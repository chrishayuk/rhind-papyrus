//! A language server for `.rmp` pages, over standard input and output
//! (`rhind lsp`). Problems as the text changes: the scribe's wrong figures as
//! warnings, lines not in the notation as errors. Hover shows a figure's
//! value, how its row is made from the rows before it, and what the working
//! gives. Colours come as semantic tokens, so the same server colours the
//! editor in VS Code and on the web.

use crate::check::{self, Mark, Run, Severity};
use crate::json::{self, obj, Json};
use crate::number::Term;
use crate::page::{Figure, Kind, Statement};
use crate::rat::Rat;
use crate::{scribe, slips};
use std::collections::HashMap;
use std::io::{BufRead, Write};

pub const TOKEN_TYPES: &[&str] = &[
    "comment",
    "string",
    "keyword",
    "number",
    "operator",
    "namespace",
    "macro",
    "parameter",
];
const T_COMMENT: usize = 0;
const T_WORDS: usize = 1;
const T_KEYWORD: usize = 2;
const T_NUMBER: usize = 3;
const T_TICK: usize = 4;
const T_HEADING: usize = 5;
const T_RED: usize = 6;
const T_RESTORED: usize = 7;

fn read_message(input: &mut impl BufRead) -> Option<Json> {
    let mut length = None;
    loop {
        let mut line = String::new();
        if input.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                length = value.trim().parse::<usize>().ok();
            }
        }
    }
    let mut body = vec![0; length?];
    input.read_exact(&mut body).ok()?;
    json::parse(&String::from_utf8_lossy(&body)).ok()
}

fn send(output: &mut impl Write, message: Json) {
    let mut body = String::new();
    message.write(&mut body);
    let _ = write!(output, "Content-Length: {}\r\n\r\n{}", body.len(), body);
    let _ = output.flush();
}

fn position(line: usize, character: usize) -> Json {
    obj([("line", line.into()), ("character", character.into())])
}

fn range(line: usize, start: usize, end: usize) -> Json {
    obj([("start", position(line, start)), ("end", position(line, end))])
}

fn line_width(run: &Run, line: usize) -> usize {
    run.lines.get(line).map(|l| l.text.encode_utf16().count()).unwrap_or(0)
}

fn diagnostic_list(run: &Run) -> Vec<Json> {
    run.findings
        .iter()
        .map(|f| {
            let (s, e) = if f.span.1 > f.span.0 {
                f.span
            } else {
                (0, line_width(run, f.line))
            };
            let mut message = f.message.clone();
            for s in slips::explain(run, f) {
                message.push_str(&format!("\ncould be {}: {}", s.model, s.explanation));
            }
            obj([
                ("range", range(f.line, s, e)),
                (
                    "severity",
                    match f.severity {
                        Severity::Notation => 1,
                        Severity::Wrong => 2,
                        Severity::Form => 3,
                        Severity::Unchecked => 4,
                    }
                    .into(),
                ),
                ("source", "rhind".into()),
                ("message", message.into()),
            ])
        })
        .collect()
}

fn diagnostics(uri: &str, text: &str) -> Json {
    let run = check::run(text);
    obj([
        ("jsonrpc", "2.0".into()),
        ("method", "textDocument/publishDiagnostics".into()),
        (
            "params",
            obj([("uri", uri.into()), ("diagnostics", diagnostic_list(&run).into())]),
        ),
    ])
}

fn capabilities() -> Json {
    obj([
        (
            "capabilities",
            obj([
                ("textDocumentSync", 1.into()),
                ("hoverProvider", true.into()),
                ("documentSymbolProvider", true.into()),
                ("foldingRangeProvider", true.into()),
                (
                    "semanticTokensProvider",
                    obj([
                        (
                            "legend",
                            obj([
                                (
                                    "tokenTypes",
                                    TOKEN_TYPES.iter().map(|t| Json::from(*t)).collect::<Vec<_>>().into(),
                                ),
                                ("tokenModifiers", Vec::new().into()),
                            ]),
                        ),
                        ("full", true.into()),
                    ]),
                ),
                ("completionProvider", obj([("resolveProvider", false.into())])),
            ]),
        ),
        (
            "serverInfo",
            obj([("name", "rhind".into()), ("version", env!("CARGO_PKG_VERSION").into())]),
        ),
    ])
}

/// (line, start, length, type), in order.
pub fn semantic(run: &Run) -> Vec<(usize, usize, usize, usize)> {
    let mut out = Vec::new();
    for l in &run.lines {
        let mut toks: Vec<(usize, usize, usize)> = Vec::new();
        let width = l.text.chars().count();
        let lead = l.text.chars().take_while(|c| c.is_whitespace()).count();
        let body_end = l.note.as_ref().map(|n| n.0).unwrap_or(width);
        let fig = |f: &Figure, toks: &mut Vec<(usize, usize, usize)>| {
            let kind = if f.red {
                T_RED
            } else if f.restored || f.lost || f.uncertain {
                T_RESTORED
            } else {
                T_NUMBER
            };
            if f.span.1 > f.span.0 {
                toks.push((f.span.0, f.span.1 - f.span.0, kind));
            }
        };
        match &l.kind {
            Kind::Heading(_) => toks.push((lead, body_end.saturating_sub(lead), T_HEADING)),
            Kind::Words(_) => toks.push((lead, body_end.saturating_sub(lead), T_WORDS)),
            Kind::Statement(s) => {
                let kw_len = match s {
                    Statement::Reckon(..) => "Reckon with".len(),
                    _ => s.keyword().len(),
                };
                toks.push((lead + l.red as usize, kw_len, T_KEYWORD));
                let figs = s.figures();
                if let (
                    Statement::Multiply(..) | Statement::Reckon(..) | Statement::Add(..) | Statement::Take(..),
                    [a, b],
                ) = (s, figs.as_slice())
                {
                    fig(a, &mut toks);
                    let sep_start = a.span.1 + 1;
                    let sep_len = b.span.0.saturating_sub(sep_start + 1);
                    if sep_len > 0 {
                        toks.push((sep_start, sep_len, T_KEYWORD));
                    }
                    fig(b, &mut toks);
                } else {
                    for f in figs {
                        fig(f, &mut toks);
                    }
                }
            }
            Kind::Row { tick_col, cells, .. } => {
                if let Some(c) = tick_col {
                    toks.push((*c, 1, T_TICK));
                }
                for c in cells {
                    fig(c, &mut toks);
                }
            }
            Kind::Figure(f) => fig(f, &mut toks),
            Kind::Bad(_) | Kind::Blank => {}
        }
        if let Some((c, _)) = &l.note {
            toks.push((*c, width - c, T_COMMENT));
        }
        toks.sort();
        // to UTF-16 columns
        let chars: Vec<char> = l.text.chars().collect();
        let u16 = |c: usize| {
            chars[..c.min(chars.len())]
                .iter()
                .map(|ch| ch.len_utf16())
                .sum::<usize>()
        };
        for (s, len, k) in toks {
            let a = u16(s);
            let b = u16(s + len);
            if b > a {
                out.push((l.no, a, b - a, k));
            }
        }
    }
    out
}

fn semantic_tokens(text: &str) -> Json {
    let run = check::run(text);
    let mut data = Vec::new();
    let (mut last_line, mut last_start) = (0, 0);
    for (line, start, length, kind) in semantic(&run) {
        let delta = if line == last_line { start - last_start } else { start };
        data.extend([line - last_line, delta, length, kind, 0].map(Json::from));
        (last_line, last_start) = (line, start);
    }
    obj([("data", data.into())])
}

fn describe_value(f: &Figure) -> String {
    match &f.value {
        Some(q) => {
            let v = q.value;
            let mut s = format!("**{}**", q);
            if !v.is_int() {
                s.push_str(&format!(" = {} ≈ {:.6}", v, v.to_f64()));
            }
            if let Some(u) = q.unit {
                s.push_str(&format!(" {}", u.name()));
            }
            s
        }
        None if f.lost => "lost: the papyrus is damaged here and nothing is restored".into(),
        None => f.error.clone().unwrap_or_default(),
    }
}

/// How a row is made from the rows above it, as the scribe would make it.
fn derivation(run: &Run, line: usize) -> Option<String> {
    let row = |i: usize| -> Option<(Rat, Rat)> {
        match &run.lines.get(i)?.kind {
            Kind::Row { cells, .. } if cells.len() >= 2 => {
                Some((cells[0].value.as_ref()?.value, cells[1].value.as_ref()?.value))
            }
            _ => None,
        }
    };
    let (l, _) = row(line)?;
    let mut i = line;
    while i > 0 && row(i - 1).is_some() {
        i -= 1;
        let (pl, _) = row(i)?;
        if pl * Rat::int(2) == l {
            return Some(format!("twice the row on line {}", i + 1));
        }
        if pl / Rat::int(2) == l {
            return Some(format!("half the row on line {}", i + 1));
        }
        if pl * Rat::new(2, 3) == l {
            return Some(format!("two-thirds of the row on line {}", i + 1));
        }
        if pl / Rat::int(10) == l {
            return Some(format!("a tenth of the row on line {}", i + 1));
        }
        if pl * Rat::int(10) == l {
            return Some(format!("ten times the row on line {}", i + 1));
        }
    }
    if l == Rat::ONE {
        return Some("the number to be multiplied, once".into());
    }
    None
}

const KEYWORDS: &[(&str, &str)] = &[
    ("Multiply", "`Multiply A by B.` The next column multiplies A by B: doublings of A, with the rows that make B ticked. Left out, rhind writes the column in."),
    ("Reckon", "`Reckon with A to find B.` Division, the scribe's way: what must A be multiplied by to make B? Double A, tick the rows that make B; the ticked multipliers are the answer."),
    ("Total", "`Total X.` What precedes adds to X: the ticked products of a column (all of them if none is ticked), or a list of figures."),
    ("Result", "`Result X.` The last multiplication or division gave X."),
    ("Remainder", "`Remainder X.` The last subtraction left X."),
    ("Add", "`Add A to B.` An addition written as a sentence."),
    ("Take", "`Take A from B.` A subtraction written as a sentence."),
    ("Proof", "`Proof.` What follows proves the answer: the scribe works the problem forward from it."),
];

pub fn hover(text: &str, line: usize, col16: usize) -> Option<String> {
    let run = check::run(text);
    let l = run.lines.get(line)?;
    let chars: Vec<char> = l.text.chars().collect();
    // UTF-16 column to char column
    let mut col = 0;
    let mut acc = 0;
    for (i, c) in chars.iter().enumerate() {
        if acc >= col16 {
            col = i;
            break;
        }
        acc += c.len_utf16();
        col = i + 1;
    }
    let inside = |f: &Figure| col >= f.span.0 && col <= f.span.1;
    let mark = |s: &mut String| {
        match &run.marks[line] {
            Mark::Right => s.push_str("\n\n✓ the working agrees"),
            Mark::Wrong(e) => s.push_str(&format!("\n\n✗ the working gives **{}**", e)),
            Mark::Unchecked => s.push_str("\n\n? not checked"),
            Mark::None => {}
        }
        for f in run
            .findings
            .iter()
            .filter(|f| f.line == line && f.severity == Severity::Wrong)
        {
            for sl in slips::explain(&run, f) {
                s.push_str(&format!("\n\ncould be *{}*: {}", sl.model, sl.explanation));
            }
            if let Some(c) = f.carried {
                s.push_str(if c {
                    "\n\nThe error is carried into what follows."
                } else {
                    "\n\nWhat follows is right: the error is not carried."
                });
            }
        }
    };
    if let Some((c, n)) = &l.note {
        if col >= *c {
            return Some(format!("A note, not in the papyrus: {}", n));
        }
    }
    match &l.kind {
        Kind::Heading(h) if h.starts_with("No.") => Some(format!("**Problem {}**", h.trim_start_matches("No. "))),
        Kind::Heading(h) => {
            let n: u64 = h.split(" ÷ ").nth(1)?.parse().ok()?;
            let parts = crate::table::twice(n).map(|p| {
                p.iter()
                    .map(|d| crate::number::overbar(*d))
                    .collect::<Vec<_>>()
                    .join(" ")
            });
            Some(format!(
                "**{}**, an entry of the table of two divided by odd numbers{}",
                h,
                parts.map(|p| format!(": {}", p)).unwrap_or_default()
            ))
        }
        Kind::Words(_) => Some("The scribe's words, in translation. Numbers in them are not computed.".into()),
        Kind::Statement(s) => {
            for f in s.figures() {
                if inside(f) {
                    let mut h = describe_value(f);
                    mark(&mut h);
                    return Some(h);
                }
            }
            let mut h = KEYWORDS.iter().find(|k| k.0 == s.keyword()).map(|k| k.1.to_string())?;
            mark(&mut h);
            Some(h)
        }
        Kind::Row { tick, tick_col, cells } => {
            if tick_col.map(|c| col <= c + 1 && col >= c).unwrap_or(false) && *tick {
                return Some("The scribe's tick: this row is one of those added to make the answer.".into());
            }
            for (k, f) in cells.iter().enumerate() {
                if inside(f) {
                    let mut h = describe_value(f);
                    if k == 0 {
                        h.push_str("\n\nthe multiplier");
                        if let Some(d) = derivation(&run, line) {
                            h.push_str(&format!(": {}", d));
                        }
                    } else if k == 1 {
                        h.push_str("\n\nthe product");
                        mark(&mut h);
                    }
                    if f.red {
                        h.push_str("\n\nIn red ink.");
                    }
                    if f.restored {
                        h.push_str("\n\nRestored: the papyrus is damaged here.");
                    }
                    return Some(h);
                }
            }
            None
        }
        Kind::Figure(f) => Some(describe_value(f)),
        Kind::Bad(e) => Some(format!("Not in the notation: {}", e)),
        Kind::Blank => None,
    }
}

fn completion() -> Json {
    let mut items: Vec<Json> = Vec::new();
    for (k, d) in KEYWORDS {
        let label = match *k {
            "Multiply" => "Multiply 3 by 4.",
            "Reckon" => "Reckon with 5 to find 15.",
            "Add" => "Add 12 to 3.",
            "Take" => "Take 9 from 10.",
            "Proof" => "Proof.",
            other => {
                items.push(obj([
                    ("label", format!("{} ", other).into()),
                    ("kind", 14.into()),
                    ("detail", (*d).into()),
                ]));
                continue;
            }
        };
        items.push(obj([
            ("label", label.into()),
            ("kind", 14.into()),
            ("detail", (*d).into()),
        ]));
    }
    for (u, _) in crate::number::UNITS {
        items.push(obj([
            ("label", (*u).into()),
            ("kind", 21.into()),
            ("detail", "a unit".into()),
        ]));
    }
    items.into()
}

fn symbols(text: &str) -> Json {
    let run = check::run(text);
    let n = run.sections.len();
    run.sections
        .iter()
        .enumerate()
        .map(|(k, s)| {
            let last = run
                .sections
                .get(k + 1)
                .map(|t| t.line.saturating_sub(1))
                .unwrap_or(run.lines.len().saturating_sub(1));
            let _ = n;
            obj([
                ("name", s.heading.clone().into()),
                ("kind", 12.into()),
                (
                    "range",
                    obj([
                        ("start", position(s.line, 0)),
                        ("end", position(last, line_width(&run, last))),
                    ]),
                ),
                ("selectionRange", range(s.line, 0, line_width(&run, s.line))),
            ])
        })
        .collect::<Vec<_>>()
        .into()
}

fn folding(text: &str) -> Json {
    let run = check::run(text);
    run.sections
        .iter()
        .enumerate()
        .filter_map(|(k, s)| {
            let last = run
                .sections
                .get(k + 1)
                .map(|t| t.line.saturating_sub(1))
                .unwrap_or(run.lines.len().saturating_sub(1));
            (last > s.line).then(|| obj([("startLine", s.line.into()), ("endLine", last.into())]))
        })
        .collect::<Vec<_>>()
        .into()
}

fn answer(method: &str, params: &Json, docs: &HashMap<String, String>) -> Option<Json> {
    let uri = params.get("textDocument").get("uri").str().unwrap_or("");
    let text = docs.get(uri).map(String::as_str).unwrap_or("");
    let line = params.get("position").get("line").num().unwrap_or(0);
    let col = params.get("position").get("character").num().unwrap_or(0);
    Some(match method {
        "initialize" => capabilities(),
        "shutdown" => Json::Null,
        "textDocument/hover" => match hover(text, line, col) {
            Some(h) => obj([("contents", obj([("kind", "markdown".into()), ("value", h.into())]))]),
            None => Json::Null,
        },
        "textDocument/semanticTokens/full" => semantic_tokens(text),
        "textDocument/documentSymbol" => symbols(text),
        "textDocument/foldingRange" => folding(text),
        "textDocument/completion" => completion(),
        _ => return None,
    })
}

pub fn serve(input: &mut impl BufRead, output: &mut impl Write) -> Result<(), String> {
    let mut docs: HashMap<String, String> = HashMap::new();
    let mut shutting_down = false;
    while let Some(message) = read_message(input) {
        let method = message.get("method").str().unwrap_or("").to_string();
        let params = message.get("params");
        let id = message.get("id").clone();
        let uri = params.get("textDocument").get("uri").str().unwrap_or("").to_string();
        match method.as_str() {
            "exit" => {
                return if shutting_down {
                    Ok(())
                } else {
                    Err("exit without shutdown".into())
                }
            }
            "textDocument/didOpen" => {
                let text = params.get("textDocument").get("text").str().unwrap_or("").to_string();
                send(output, diagnostics(&uri, &text));
                docs.insert(uri, text);
            }
            "textDocument/didChange" => {
                if let Some(text) = params
                    .get("contentChanges")
                    .items()
                    .last()
                    .and_then(|c| c.get("text").str())
                {
                    send(output, diagnostics(&uri, text));
                    docs.insert(uri, text.to_string());
                }
            }
            "textDocument/didClose" => {
                docs.remove(&uri);
                send(output, diagnostics(&uri, ""));
            }
            _ if id == Json::Null => {}
            _ => {
                shutting_down |= method == "shutdown";
                let reply = match answer(&method, params, &docs) {
                    Some(result) => obj([("jsonrpc", "2.0".into()), ("id", id), ("result", result)]),
                    None => obj([
                        ("jsonrpc", "2.0".into()),
                        ("id", id),
                        (
                            "error",
                            obj([
                                ("code", Json::Num(-32601.0)),
                                ("message", format!("method not found: {}", method).into()),
                            ]),
                        ),
                    ]),
                };
                send(output, reply);
            }
        }
    }
    Ok(())
}

/// A run as JSON, for the playground: the page as rhind prints it, each line
/// with its mark, the working written in, and what was found.
pub fn run_json(text: &str) -> String {
    let run = check::run(text);
    let lines: Vec<Json> = run
        .lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let (mark, expected) = match &run.marks[i] {
                Mark::Right => ("right", String::new()),
                Mark::Wrong(e) => ("wrong", e.clone()),
                Mark::Unchecked => ("unchecked", String::new()),
                Mark::None => ("", String::new()),
            };
            let kind = match &l.kind {
                Kind::Heading(_) => "heading",
                Kind::Words(_) => "words",
                Kind::Statement(_) => "statement",
                Kind::Row { tick: true, .. } => "row ticked",
                Kind::Row { .. } => "row",
                Kind::Figure(_) => "figure",
                Kind::Bad(_) => "bad",
                Kind::Blank => "blank",
            };
            let (left, right) = match &l.kind {
                Kind::Row { cells, .. } => (
                    cells
                        .first()
                        .and_then(|c| c.value.as_ref())
                        .map(|q| q.to_string())
                        .unwrap_or_default(),
                    cells
                        .get(1)
                        .and_then(|c| c.value.as_ref())
                        .map(|q| q.to_string())
                        .unwrap_or_default(),
                ),
                _ => (String::new(), String::new()),
            };
            let red = l.red || matches!(&l.kind, Kind::Row { cells, .. } if cells.iter().any(|c| c.red));
            obj([
                (
                    "text",
                    {
                        let body: String = match &l.note {
                            Some((c, _)) => l.text.chars().take(*c).collect(),
                            None => l.text.clone(),
                        };
                        check::egyptian(body.trim_end())
                    }
                    .into(),
                ),
                ("note", l.note.as_ref().map(|n| n.1.clone()).unwrap_or_default().into()),
                ("kind", kind.into()),
                ("mark", mark.into()),
                ("expected", expected.into()),
                ("left", left.into()),
                ("right", right.into()),
                ("red", red.into()),
                ("how", derivation(&run, i).unwrap_or_default().into()),
            ])
        })
        .collect();
    let written: Vec<Json> = run
        .written_in
        .iter()
        .map(|(at, rows)| {
            obj([
                ("after", (*at).into()),
                (
                    "rows",
                    rows.iter().map(|r| Json::from(r.as_str())).collect::<Vec<_>>().into(),
                ),
            ])
        })
        .collect();
    let findings: Vec<Json> = run
        .findings
        .iter()
        .map(|f| {
            obj([
                ("line", f.line.into()),
                (
                    "severity",
                    match f.severity {
                        Severity::Wrong => "wrong",
                        Severity::Form => "form",
                        Severity::Unchecked => "unchecked",
                        Severity::Notation => "notation",
                    }
                    .into(),
                ),
                ("message", f.message.clone().into()),
                ("carried", f.carried.map(Json::Bool).unwrap_or(Json::Null)),
                (
                    "slips",
                    slips::explain(&run, f)
                        .into_iter()
                        .map(|s| obj([("model", s.model.into()), ("explanation", s.explanation.into())]))
                        .collect::<Vec<_>>()
                        .into(),
                ),
            ])
        })
        .collect();
    let (checked, right) = run.marks.iter().fold((0, 0), |a, m| match m {
        Mark::Right => (a.0 + 1, a.1 + 1),
        Mark::Wrong(_) => (a.0 + 1, a.1),
        _ => a,
    });
    let mut out = String::new();
    obj([
        ("lines", lines.into()),
        ("written", written.into()),
        ("findings", findings.into()),
        ("checked", checked.into()),
        ("right", right.into()),
    ])
    .write(&mut out);
    let _ = Term::TwoThirds;
    let _ = scribe::express;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hover_and_tokens() {
        let t = "No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\n";
        let h = hover(t, 3, 7).unwrap();
        assert!(h.contains("14"), "{}", h);
        let h = hover(t, 2, 3).unwrap();
        assert!(h.contains("twice"), "{}", h);
        let toks = semantic(&check::run(t));
        assert!(toks.iter().any(|t| t.3 == T_HEADING));
        assert!(toks.iter().any(|t| t.3 == T_TICK));
    }
}
