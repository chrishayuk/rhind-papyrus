//! The papyrus itself: every problem as transcribed, run, and what running it
//! finds set against what the editions say.

use crate::check::{self, Finding, Run, Severity};
use crate::slips::{self, Slip};
use crate::table;

include!(concat!(env!("OUT_DIR"), "/papyrus.rs"));

/// Papyrus order: the tables, then the problems by number.
fn order(name: &str) -> (u32, u32, String) {
    if let Some(t) = name.strip_prefix("table-") {
        let k = if t.starts_with("2n") { 0 } else { 1 };
        return (0, k, t.to_string());
    }
    let id = name.trim_start_matches("no-");
    let num: String = id.chars().take_while(|c| c.is_ascii_digit()).collect();
    let rest: String = id.chars().skip_while(|c| c.is_ascii_digit()).collect();
    (1, num.parse().unwrap_or(999), rest)
}

pub fn files() -> Vec<(&'static str, &'static str)> {
    let mut v: Vec<_> = PAPYRUS.to_vec();
    v.sort_by_key(|(n, _)| order(n));
    v
}

pub fn get(name: &str) -> Option<&'static str> {
    let want = name.trim_end_matches(".rmp");
    let alt = format!("no-{:0>2}", want.trim_start_matches("no-").to_lowercase());
    PAPYRUS.iter().find(|(n, _)| *n == want || *n == alt).map(|(_, t)| *t)
}

/// Does a transcriber's note mention an edition commenting on the scribe
/// here? A keyword search over the notes, nothing more: it misses comments
/// the notes do not quote, or quote in other words, and it can catch a
/// comment about an edition rather than the papyrus. It is not a census of
/// what Chace and Peet say.
pub fn flagged(note: &str) -> bool {
    let n = note.to_lowercase();
    [
        "sic",
        "error",
        "should be",
        "mistake",
        "wrong",
        "erroneous",
        "incorrect",
        "blunder",
        "careless",
        "dot",
        "used here",
        "intended",
        "omitted",
        "written for",
        "was written",
        "left off",
        "read ",
    ]
    .iter()
    .any(|w| n.contains(w))
        && !n.contains("no error")
}

/// What kind of discrepancy a diagnostic record is. This sorts records by
/// the explanations that fit; it says nothing about which is the cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// only a part sign differs (3̅0̅ written 30): Chace (vol. II p. xi) says
    /// an omission clear from the context need not be an error
    Notation,
    /// the rows ticked do not make the total written
    Selection,
    /// anything else: a figure that differs in more than its part sign
    Figure,
}

pub fn kind(slips: &[Slip]) -> Kind {
    if !slips.is_empty() && slips.iter().all(|s| s.model == "part-sign") {
        Kind::Notation
    } else if !slips.is_empty() && slips.iter().all(|s| s.model == "ticks") {
        Kind::Selection
    } else {
        Kind::Figure
    }
}

pub struct Wrong {
    pub kind: Kind,
    pub file: &'static str,
    pub heading: String,
    pub finding: Finding,
    pub slips: Vec<Slip>,
    pub noted: bool,
    pub note: Option<String>,
}

pub struct Report {
    pub runs: Vec<(&'static str, Run)>,
    pub wrong: Vec<Wrong>,
    /// lines an edition flags where rhind found nothing wrong: file, line,
    /// heading, note, and whether rhind checked the line (and found it right)
    pub unmatched: Vec<(&'static str, usize, String, String, bool)>,
}

fn heading_at(run: &Run, line: usize) -> String {
    run.sections
        .iter()
        .rev()
        .find(|s| s.line <= line)
        .map(|s| s.heading.clone())
        .unwrap_or_default()
}

pub fn verify() -> Report {
    let mut runs = Vec::new();
    let mut wrong = Vec::new();
    let mut unmatched = Vec::new();
    for (name, text) in files() {
        let run = check::run(text);
        let mut wrong_lines = std::collections::HashSet::new();
        for f in run.findings.iter().filter(|f| f.severity == Severity::Wrong) {
            wrong_lines.insert(f.line);
            let found = slips::explain(&run, f);
            // the note is on the line of the figure, or on the line a slip points to
            let mut note = run.lines[f.line]
                .note
                .as_ref()
                .map(|n| n.1.clone())
                .filter(|n| flagged(n));
            for s in &found {
                // every line a slip names
                let mut named: Vec<usize> = s.line.into_iter().collect();
                for w in s.explanation.split("line ").skip(1) {
                    if let Ok(n) = w
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                    {
                        named.push(n - 1);
                    }
                }
                for l in named {
                    if l >= run.lines.len() {
                        continue;
                    }
                    wrong_lines.insert(l);
                    if note.is_none() {
                        note = run.lines[l].note.as_ref().map(|n| n.1.clone()).filter(|n| flagged(n));
                    }
                    if s.model == "ticks" || s.model == "in-the-list" {
                        wrong_lines.insert(l);
                    }
                }
            }
            let noted = note.is_some();
            wrong.push(Wrong {
                kind: kind(&found),
                file: name,
                heading: heading_at(&run, f.line),
                finding: f.clone(),
                slips: found,
                noted,
                note,
            });
        }
        for l in &run.lines {
            if let Some((_, note)) = &l.note {
                if flagged(note) && !wrong_lines.contains(&l.no) {
                    let checked = run.marks[l.no] == check::Mark::Right;
                    unmatched.push((name, l.no, heading_at(&run, l.no), note.clone(), checked));
                }
            }
        }
        runs.push((name, run));
    }
    Report { runs, wrong, unmatched }
}

/// The 2 ÷ n table as transcribed, against the table the engine doubles with.
pub fn table_differences() -> Vec<(u64, Vec<u64>, Vec<u64>)> {
    let mut out = Vec::new();
    for (name, text) in files() {
        if !name.starts_with("table-2n") {
            continue;
        }
        let run = check::run(text);
        for (k, s) in run.sections.iter().enumerate() {
            let Some(n) = s.heading.strip_prefix("2 ÷ ").and_then(|n| n.parse::<u64>().ok()) else {
                continue;
            };
            let end = run.sections.get(k + 1).map(|s| s.line).unwrap_or(run.lines.len());
            let mut parts = Vec::new();
            for l in &run.lines[s.line..end] {
                if let crate::page::Kind::Row { cells, .. } = &l.kind {
                    // the summary at the head of the entry: the red parts of n
                    if let Some(q) = cells.first().filter(|c| c.red).and_then(|c| c.value.as_ref()) {
                        let e = q.eg();
                        if e.terms.len() == 1 && e.whole == 0 {
                            parts.extend(e.parts());
                        } else if e.terms.is_empty() && e.whole > 1 {
                            // written without its part sign
                            parts.push(e.whole);
                        }
                    }
                }
            }
            let want = table::twice(n).map(|p| p.to_vec()).unwrap_or_default();
            if n > 3 && parts != want {
                out.push((n, want, parts));
            }
        }
    }
    out
}
