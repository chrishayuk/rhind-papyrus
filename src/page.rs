//! Reading a `.rmp` page: each line is a heading, the scribe's words, a
//! statement, a row of a column, a single figure, or nothing.

use crate::number::{self, Quantity};

/// A figure as written, with what the transcription says about the ink and
/// the state of the papyrus.
#[derive(Clone, Debug, PartialEq)]
pub struct Figure {
    pub text: String,
    /// character columns in the line, start and end
    pub span: (usize, usize),
    /// `None` if lost, or not a number
    pub value: Option<Quantity>,
    pub error: Option<String>,
    pub red: bool,
    pub restored: bool,
    pub lost: bool,
    pub uncertain: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Multiply(Figure, Figure),
    Reckon(Figure, Figure),
    Total(Figure),
    Result(Figure),
    Remainder(Figure),
    Add(Figure, Figure),
    Take(Figure, Figure),
    Proof,
}

impl Statement {
    pub fn figures(&self) -> Vec<&Figure> {
        match self {
            Statement::Multiply(a, b) | Statement::Reckon(a, b) | Statement::Add(a, b) | Statement::Take(a, b) => {
                vec![a, b]
            }
            Statement::Total(a) | Statement::Result(a) | Statement::Remainder(a) => vec![a],
            Statement::Proof => vec![],
        }
    }
    pub fn keyword(&self) -> &'static str {
        match self {
            Statement::Multiply(..) => "Multiply",
            Statement::Reckon(..) => "Reckon",
            Statement::Total(..) => "Total",
            Statement::Result(..) => "Result",
            Statement::Remainder(..) => "Remainder",
            Statement::Add(..) => "Add",
            Statement::Take(..) => "Take",
            Statement::Proof => "Proof",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Blank,
    /// `No. 26`, or `2 ÷ 7` in the table
    Heading(String),
    Words(String),
    Statement(Statement),
    Row {
        tick: bool,
        tick_col: Option<usize>,
        cells: Vec<Figure>,
    },
    Figure(Figure),
    /// a line that is none of these: the reason
    Bad(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    /// zero-based
    pub no: usize,
    pub text: String,
    pub kind: Kind,
    /// the `%` note and the column it starts at
    pub note: Option<(usize, String)>,
    /// the whole line is in red: `{Total 21.}`
    pub red: bool,
}

/// Take the marks off a figure and read it.
pub fn figure(text: &str, start: usize) -> Figure {
    let len = text.chars().count();
    let red = text.contains('{');
    let restored = text.contains('[');
    let uncertain = text.contains('?');
    let lost = text.contains("...") || text.contains('…');
    let clean: String = text.chars().filter(|c| !"{}[]?".contains(*c)).collect();
    let (value, error) = if lost {
        (None, None)
    } else {
        match number::parse(&clean) {
            Ok(q) => (Some(q), None),
            Err(e) => (None, Some(e)),
        }
    };
    Figure {
        text: text.to_string(),
        span: (start, start + len),
        value,
        error,
        red,
        restored,
        lost,
        uncertain,
    }
}

/// Split a row into cells at runs of two or more spaces, or tabs, keeping
/// where each starts.
fn cells(s: &str, offset: usize) -> Vec<(usize, String)> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }
        let start = i;
        loop {
            if i >= chars.len() || chars[i] == '\t' {
                break;
            }
            if chars[i] == ' ' && (i + 1 >= chars.len() || chars[i + 1] == ' ' || chars[i + 1] == '\t') {
                break;
            }
            i += 1;
        }
        out.push((offset + start, chars[start..i].iter().collect()));
    }
    out
}

/// Find `needle` in `hay` at a char offset, returning the char column.
fn col_of(hay: &str, byte: usize) -> usize {
    hay[..byte].chars().count()
}

fn statement(body: &str, at: usize) -> Option<Result<Statement, String>> {
    let s = body.trim_end();
    let first = s.split_whitespace().next()?.trim_end_matches('.');
    let known = [
        "Multiply",
        "Reckon",
        "Total",
        "Result",
        "Remainder",
        "Add",
        "Take",
        "Proof",
    ];
    if !known.contains(&first) {
        return None;
    }
    let Some(inner) = s.strip_suffix('.') else {
        return Some(Err(format!("a statement ends with a full stop: `{}.`", s)));
    };
    let fig = |from: usize, to: usize| -> Figure {
        let t = &inner[from..to];
        let lead = t.len() - t.trim_start().len();
        figure(t.trim(), at + col_of(s, from + lead))
    };
    let after = |kw: &str| inner[kw.len()..].len();
    let _ = after;
    let res = match first {
        "Proof" => {
            if inner.trim() == "Proof" {
                Ok(Statement::Proof)
            } else {
                Err("`Proof.` stands alone".into())
            }
        }
        "Total" | "Result" | "Remainder" => {
            let f = fig(first.len(), inner.len());
            if f.text.is_empty() {
                Err(format!("`{}` needs a number: `{} 15.`", first, first))
            } else {
                Ok(match first {
                    "Total" => Statement::Total(f),
                    "Result" => Statement::Result(f),
                    _ => Statement::Remainder(f),
                })
            }
        }
        _ => {
            let (a, sep, b_kw) = match first {
                "Multiply" => (
                    "Multiply",
                    " by ",
                    Statement::Multiply as fn(Figure, Figure) -> Statement,
                ),
                "Reckon" => (
                    "Reckon with",
                    " to find ",
                    Statement::Reckon as fn(Figure, Figure) -> Statement,
                ),
                "Add" => ("Add", " to ", Statement::Add as fn(Figure, Figure) -> Statement),
                _ => ("Take", " from ", Statement::Take as fn(Figure, Figure) -> Statement),
            };
            if !inner.starts_with(a) {
                return Some(Err(format!("write `{} … {} ….`", a, sep.trim())));
            }
            match inner.find(sep) {
                Some(p) if p > a.len() => Ok(b_kw(fig(a.len(), p), fig(p + sep.len(), inner.len()))),
                _ => Err(format!("write `{} …{}….`", a, sep)),
            }
        }
    };
    Some(res)
}

fn heading(s: &str) -> Option<String> {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("No. ") {
        let id = rest.trim();
        if !id.is_empty()
            && id.chars().next().unwrap().is_ascii_digit()
            && id.chars().all(|c| c.is_ascii_alphanumeric())
        {
            return Some(format!("No. {}", id));
        }
    }
    for sep in ['÷', ':'] {
        if let Some((a, b)) = t.split_once(sep) {
            let (a, b) = (a.trim(), b.trim());
            if !a.is_empty()
                && !b.is_empty()
                && a.chars().all(|c| c.is_ascii_digit())
                && b.chars().all(|c| c.is_ascii_digit())
            {
                return Some(format!("{} ÷ {}", a, b));
            }
        }
    }
    None
}

/// Find the `%` that starts a note: not inside double quotes.
fn note_at(line: &str) -> Option<usize> {
    let mut quoted = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' | '“' | '”' => quoted = !quoted,
            '%' if !quoted => return Some(i),
            _ => {}
        }
    }
    None
}

pub fn read_line(no: usize, raw: &str) -> Line {
    let (body, note) = match note_at(raw) {
        Some(b) => (&raw[..b], Some((col_of(raw, b), raw[b + 1..].trim().to_string()))),
        None => (raw, None),
    };
    let mut line = Line {
        no,
        text: raw.to_string(),
        kind: Kind::Blank,
        note,
        red: false,
    };
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return line;
    }
    let lead = col_of(body, body.len() - body.trim_start().len());
    // a whole line in red: {Total 21.}
    let (inner, inner_at) = if trimmed.starts_with('{') && trimmed.ends_with('}') && trimmed.matches('{').count() == 1 {
        line.red = true;
        (&trimmed[1..trimmed.len() - 1], lead + 1)
    } else {
        (trimmed, lead)
    };
    // a word of a statement in red: {Total} 5 2̅ 8̅ setat.
    let mut owned = String::new();
    let mut inner = inner;
    let mut inner_at = inner_at;
    if let Some(rest) = inner.strip_prefix('{') {
        if let Some((word, after)) = rest.split_once('}') {
            if ["Total", "Result", "Remainder", "Proof"].contains(&word) {
                owned = format!("{}{}", word, after);
                inner_at += 0;
                line.red = true;
            }
        }
    }
    if !owned.is_empty() {
        inner = &owned;
        let _ = &mut inner_at;
    }
    if let Some(h) = heading(inner) {
        line.kind = Kind::Heading(h);
        return line;
    }
    let unbraced = inner.trim_start_matches('{');
    if unbraced.starts_with('"') || unbraced.starts_with('“') {
        if unbraced.len() != inner.len() {
            line.red = true;
        }
        let w = unbraced.trim_matches(|c| c == '"' || c == '“' || c == '”' || c == '{' || c == '}');
        line.kind = Kind::Words(w.to_string());
        return line;
    }
    if let Some(st) = statement(inner, inner_at) {
        line.kind = match st {
            Ok(mut s) => {
                if line.red {
                    match &mut s {
                        Statement::Total(f) | Statement::Result(f) | Statement::Remainder(f) => f.red = true,
                        _ => {}
                    }
                }
                Kind::Statement(s)
            }
            Err(e) => Kind::Bad(e),
        };
        return line;
    }
    // a row, or one figure
    let (tick, rest, rest_at, tick_col) = if let Some(r) = inner.strip_prefix('\\').or_else(|| inner.strip_prefix('✓'))
    {
        (true, r, inner_at + 1, Some(inner_at))
    } else {
        (false, inner, inner_at, None)
    };
    let cs = cells(rest, rest_at);
    let figs: Vec<Figure> = cs.iter().map(|(at, t)| figure(t, *at)).collect();
    if let Some(bad) = figs.iter().find(|f| f.error.is_some()) {
        // a word the scribe wrote, left unquoted
        line.kind = Kind::Bad(bad.error.clone().unwrap());
        return line;
    }
    line.kind = match figs.len() {
        0 => Kind::Blank,
        1 if !tick => Kind::Figure(figs.into_iter().next().unwrap()),
        _ => Kind::Row {
            tick,
            tick_col,
            cells: figs,
        },
    };
    line
}

pub fn read(text: &str) -> Vec<Line> {
    text.lines().enumerate().map(|(i, l)| read_line(i, l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kinds() {
        assert!(matches!(read_line(0, "No. 26").kind, Kind::Heading(_)));
        assert!(matches!(read_line(0, "2 ÷ 7").kind, Kind::Heading(_)));
        assert!(matches!(read_line(0, "\"A quantity\"").kind, Kind::Words(_)));
        assert!(matches!(
            read_line(0, "Reckon with 5 to find 15.").kind,
            Kind::Statement(Statement::Reckon(..))
        ));
        match read_line(0, "\\ 4   15     % Peet: (sic)").kind {
            Kind::Row { tick, cells, .. } => {
                assert!(tick);
                assert_eq!(cells.len(), 2);
                assert_eq!(cells[1].text, "15");
            }
            k => panic!("{:?}", k),
        }
        match read_line(0, " '4   1 '2 '4").kind {
            Kind::Row { cells, .. } => assert_eq!(cells[1].text, "1 '2 '4"),
            k => panic!("{:?}", k),
        }
        let l = read_line(0, "{Total 21.}");
        assert!(l.red);
        assert!(matches!(read_line(0, "Total 21").kind, Kind::Bad(_)));
        match read_line(0, "  [1]   {5}").kind {
            Kind::Row { cells, .. } => {
                assert!(cells[0].restored);
                assert!(cells[1].red);
            }
            k => panic!("{:?}", k),
        }
    }
}
