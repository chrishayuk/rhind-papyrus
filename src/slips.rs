//! When the scribe's figure is wrong, what slip would have made it?
//!
//! Each model below is a way a figure can go wrong in copying or working.
//! For a wrong figure, every model that turns the right figure into the one
//! written is reported. More than one may fit; none may. A fit is evidence
//! for how the error happened, not proof.

use crate::check::{Finding, Run, Severity};
use crate::number::{Eg, Term};
use crate::page::Kind;
use crate::rat::Rat;
use crate::scribe;

#[derive(Clone, Debug, PartialEq)]
pub struct Slip {
    pub model: &'static str,
    pub explanation: String,
    /// the line the slip is on, where it is not the line of the finding
    pub line: Option<usize>,
}

pub const MODELS: &[(&str, &str)] = &[
    ("one-sign", "the whole numbers differ in one decimal place only: one hieratic sign written for another (each of 1-9, 10-90, 100-900 has its own sign)"),
    ("one-part", "the whole numbers agree and one unit fraction is missing, extra, or a different one"),
    ("part-sign", "the sign for \"part\" over a number is left off, so n̅ is written n (or added where it does not belong)"),
    ("in-the-list", "a total of a list is right, and one figure in the list is wrong by one of the slips above"),
    ("neighbour", "the figure is the one in the row above or below: the eye slipped a line"),
    ("wrong-row", "the figure is double or half of another row in the column: the wrong row was doubled or halved"),
    ("whole-only", "the figure is the right figure's whole part, or its fractions: half of it was left off"),
    ("ticks", "the total is right for other rows than those ticked: a tick left off, or put on the wrong row"),
    ("carried", "the row is right but a total or result after it is computed from a wrong row"),
];

fn digits(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut n = n;
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

pub fn one_sign(e: u64, w: u64) -> Option<String> {
    if e == w {
        return None;
    }
    let (a, b) = (digits(e), digits(w));
    let len = a.len().max(b.len());
    let mut diff = Vec::new();
    for i in 0..len {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        if x != y {
            diff.push((i, x, y));
        }
    }
    if diff.len() == 1 {
        let (place, x, y) = diff[0];
        let unit = [1u64, 10, 100, 1000, 10000][place.min(4)];
        Some(if y == 0 {
            format!("the sign for {} left out", x * unit)
        } else if x == 0 {
            format!("a sign for {} added", y * unit)
        } else {
            format!("{} written for {}", y * unit, x * unit)
        })
    } else {
        None
    }
}

fn one_part(e: &Eg, w: &Eg) -> Option<String> {
    if e.whole != w.whole {
        return None;
    }
    let (a, b): (Vec<Term>, Vec<Term>) = (e.terms.clone(), w.terms.clone());
    let extra: Vec<&Term> = b.iter().filter(|t| !a.contains(t)).collect();
    let missing: Vec<&Term> = a.iter().filter(|t| !b.contains(t)).collect();
    let name = |t: &Term| {
        Eg {
            whole: 0,
            terms: vec![*t],
        }
        .to_string()
    };
    match (missing.len(), extra.len()) {
        (1, 0) => Some(format!("{} left out", name(missing[0]))),
        (0, 1) => Some(format!("{} added", name(extra[0]))),
        (1, 1) => Some(format!("{} written for {}", name(extra[0]), name(missing[0]))),
        _ => None,
    }
}

/// The values of the figures in the column around a line.
fn column_values(run: &Run, line: usize) -> Vec<(usize, Rat, Rat)> {
    let mut start = line;
    while start > 0 && matches!(run.lines[start - 1].kind, Kind::Row { .. }) {
        start -= 1;
    }
    let mut end = line;
    while end + 1 < run.lines.len() && matches!(run.lines[end + 1].kind, Kind::Row { .. }) {
        end += 1;
    }
    (start..=end)
        .filter_map(|i| match &run.lines[i].kind {
            Kind::Row { cells, .. } if cells.len() >= 2 => {
                let l = cells[0].value.as_ref()?.value;
                let r = cells[1].value.as_ref()?.value;
                Some((i, l, r))
            }
            _ => None,
        })
        .collect()
}

/// n̅ written as n, or n as n̅: compare the terms with the part sign taken off.
pub fn part_sign(e: &Eg, w: &Eg) -> Option<String> {
    let flat = |x: &Eg| -> Vec<u64> {
        let mut v: Vec<u64> = x.parts();
        v.extend(x.terms.iter().filter_map(|t| match t {
            Term::Bare(b) => Some(*b),
            _ => None,
        }));
        if x.whole > 0 {
            v.push(x.whole);
        }
        v.sort();
        v
    };
    if e.terms.contains(&Term::TwoThirds) != w.terms.contains(&Term::TwoThirds) || flat(e) != flat(w) {
        return None;
    }
    let bare = w.terms.iter().any(|t| matches!(t, Term::Bare(_)));
    if e.whole == w.whole && !bare {
        return None;
    }
    if let Some(b) = w.terms.iter().find_map(|t| match t {
        Term::Bare(b) if e.parts().contains(b) => Some(*b),
        _ => None,
    }) {
        return Some(format!(
            "{} written for {}: the part sign left off",
            b,
            crate::number::overbar(b)
        ));
    }
    if w.whole > 0 && e.parts().contains(&w.whole) {
        Some(format!(
            "{} written for {}: the part sign left off",
            w.whole,
            crate::number::overbar(w.whole)
        ))
    } else if e.whole > 0 {
        Some(format!(
            "{} written for {}: a part sign added",
            crate::number::overbar(e.whole),
            e.whole
        ))
    } else {
        None
    }
}

/// The values a figure would have with one of its whole numbers given back
/// its part sign: `9` → 9̅, `2/3 '10 30` → ⅔ 1̅0̅ 3̅0̅.
pub fn with_part_signs(e: &Eg) -> Vec<Rat> {
    let mut out = Vec::new();
    let rest: Rat = e.terms.iter().map(|t| t.value()).sum();
    if e.whole > 1 {
        out.push(rest + Rat::part(e.whole));
    }
    for (i, t) in e.terms.iter().enumerate() {
        if let Term::Bare(b) = t {
            if *b > 1 {
                let others: Rat = e
                    .terms
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, t)| t.value())
                    .sum();
                out.push(Rat::int(e.whole as i128) + others + Rat::part(*b));
            }
        }
    }
    out
}

/// The row is right for a multiplier one slip away from the one written:
/// 2̅2̅6̅ written where 2̅3̅6̅ gives the product.
pub fn multiplier_slip(written: &Eg, wanted: Rat) -> Option<String> {
    if wanted <= Rat::ZERO || wanted.denom() > 1_000_000 {
        return None;
    }
    let we = scribe::express(wanted);
    // a single part, its number miswritten
    if let ([Term::Part(a)], [Term::Part(b)]) = (written.terms.as_slice(), we.terms.as_slice()) {
        if written.whole == 0 && we.whole == 0 {
            return one_sign(*b, *a).map(|s| {
                format!(
                    "the multiplier: {} (so the product is that of {})",
                    s.replace(&b.to_string(), &crate::number::overbar(*b))
                        .replace(&a.to_string(), &crate::number::overbar(*a)),
                    we
                )
            });
        }
    }
    if written.terms.is_empty() && we.whole == 0 && we.terms.len() == 1 {
        if let Term::Part(b) = we.terms[0] {
            if let Some(s) = one_sign(b, written.whole) {
                return Some(format!(
                    "the multiplier: {}, and its part sign left off (the product is that of {})",
                    s, we
                ));
            }
        }
    }
    None
}

/// A wrong sum of a list: which one figure, changed by a slip, makes it right?
fn in_the_list(run: &Run, f: &Finding) -> Vec<Slip> {
    let (Some(e), Some(w)) = (f.expected, f.written) else {
        return vec![];
    };
    let mut out: Vec<(bool, Slip)> = Vec::new();
    for &line in &f.items {
        let Some((item, written)) = (match &run.lines[line].kind {
            Kind::Figure(fig) => fig.value.as_ref().map(|q| (q.value, q.eg().clone())),
            _ => None,
        }) else {
            continue;
        };
        // the figure that would make the written total right
        let needed = item + (w - e);
        if needed < Rat::ZERO {
            continue;
        }
        let (ie, ne) = (written, scribe::express(needed));
        let why = if item.fract() == needed.fract() {
            one_sign(needed.floor() as u64, item.floor() as u64)
        } else {
            None
        }
        .or_else(|| part_sign(&ne, &ie))
        .or_else(|| one_part(&ne, &ie));
        if let Some(why) = why {
            // is the right figure written elsewhere on the page?
            let seen = run.lines.iter().find(|l| {
                l.no != line
                    && match &l.kind {
                        Kind::Statement(st) => st
                            .figures()
                            .iter()
                            .any(|g| g.value.as_ref().map(|q| q.value) == Some(needed)),
                        Kind::Row { cells, .. } => {
                            cells.get(1).and_then(|c| c.value.as_ref()).map(|q| q.value) == Some(needed)
                        }
                        _ => false,
                    }
            });
            let mut explanation = format!(
                "line {}: {} ({} for {}); with {} the total is right",
                line + 1,
                why,
                ie,
                ne,
                ne
            );
            if let Some(l) = seen {
                explanation.push_str(&format!(", and {} is what line {} has", ne, l.no + 1));
            }
            out.push((
                seen.is_some(),
                Slip {
                    model: "in-the-list",
                    explanation,
                    line: Some(line),
                },
            ));
        }
    }
    // where the page itself has the right figure, that is the one
    if out.iter().any(|(seen, _)| *seen) {
        out.retain(|(seen, _)| *seen);
    }
    out.into_iter().map(|(_, s)| s).collect()
}

/// The figure a finding is about, as written.
fn written_figure(run: &Run, f: &Finding) -> Option<Eg> {
    let figs: Vec<&crate::page::Figure> = match &run.lines.get(f.line)?.kind {
        Kind::Row { cells, .. } => cells.iter().collect(),
        Kind::Statement(st) => st.figures(),
        Kind::Figure(fig) => vec![fig],
        _ => vec![],
    };
    let fig = figs.into_iter().find(|g| g.span == f.span)?;
    let q = fig.value.as_ref()?;
    (q.segments.len() == 1).then(|| q.eg().clone())
}

pub fn explain(run: &Run, f: &Finding) -> Vec<Slip> {
    if !f.items.is_empty() {
        return in_the_list(run, f);
    }
    if let Some((model, explanation)) = &f.hint {
        return vec![Slip {
            model,
            explanation: explanation.clone(),
            line: None,
        }];
    }
    let mut out = Vec::new();
    if f.severity != Severity::Wrong {
        return out;
    }
    let (Some(e), Some(w)) = (f.expected, f.written) else {
        return out;
    };
    if e < Rat::ZERO || w < Rat::ZERO {
        return out;
    }
    let ee = scribe::express(e);
    let we = scribe::express(w);
    // the figure as the scribe wrote it: give one of its numbers back its
    // part sign, and see if it is right then
    if let Some(written) = written_figure(run, f) {
        // in the order with_part_signs gives them: the whole number, then the rest
        let mut nums: Vec<u64> = Vec::new();
        if written.whole > 1 {
            nums.push(written.whole);
        }
        nums.extend(written.terms.iter().filter_map(|t| {
            if let Term::Bare(b) = t {
                (*b > 1).then_some(*b)
            } else {
                None
            }
        }));
        for (v, n) in with_part_signs(&written).into_iter().zip(nums) {
            if v == e {
                return vec![Slip {
                    model: "part-sign",
                    explanation: format!(
                        "{} written for {}: the part sign left off",
                        n,
                        crate::number::overbar(n)
                    ),
                    line: None,
                }];
            }
        }
    }
    if e.fract() == w.fract() {
        if let Some(s) = one_sign(e.floor() as u64, w.floor() as u64) {
            out.push(Slip {
                model: "one-sign",
                explanation: s,
                line: None,
            });
        }
    }
    if let Some(s) = part_sign(&ee, &we) {
        out.push(Slip {
            model: "part-sign",
            explanation: s,
            line: None,
        });
    }
    if let Some(s) = one_part(&ee, &we) {
        out.push(Slip {
            model: "one-part",
            explanation: s,
            line: None,
        });
    }
    let col = column_values(run, f.line);
    for (i, _, r) in &col {
        if (*i + 1 == f.line || *i == f.line + 1) && *r == w {
            out.push(Slip {
                model: "neighbour",
                explanation: format!("the row {} has {}", if *i < f.line { "above" } else { "below" }, we),
                line: None,
            });
        }
    }
    for (i, _, r) in &col {
        if *i == f.line {
            continue;
        }
        if *r * Rat::int(2) == w {
            out.push(Slip {
                model: "wrong-row",
                explanation: format!("twice the row on line {} ({})", i + 1, scribe::express(*r)),
                line: None,
            });
        } else if *r / Rat::int(2) == w {
            out.push(Slip {
                model: "wrong-row",
                explanation: format!("half the row on line {} ({})", i + 1, scribe::express(*r)),
                line: None,
            });
        }
    }
    if !e.is_int() && (w == Rat::int(e.floor()) || w == e.fract()) && !w.is_zero() {
        out.push(Slip {
            model: "whole-only",
            explanation: format!("{} is part of {}", we, ee),
            line: None,
        });
    }
    if f.carried == Some(true) {
        out.push(Slip {
            model: "carried",
            explanation: "the total after it is computed from this figure as written".into(),
            line: None,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fifteen_for_fourteen() {
        let r = crate::check::run("\\ 1   3 '2\n  2   7\n\\ 4   15\nTotal 17 '2.\n");
        let f = r.findings.iter().find(|f| f.severity == Severity::Wrong).unwrap();
        let s = explain(&r, f);
        assert!(
            s.iter()
                .any(|s| s.model == "one-sign" && s.explanation == "5 written for 4"),
            "{:?}",
            s
        );
    }
    #[test]
    fn dropped_part_sign() {
        let r = crate::check::run("Multiply 2/3 by '3.\nResult 6 '18.\n");
        let f = r.findings.iter().find(|f| f.severity == Severity::Wrong).unwrap();
        let s = explain(&r, f);
        assert!(s.iter().any(|s| s.model == "part-sign"), "{:?}", s);
    }
    #[test]
    fn the_wrong_figure_in_a_list() {
        let r = crate::check::run("266 2/3\n200\n113 '3\n100\nTotal 700.\n");
        let f = r.findings.iter().find(|f| f.severity == Severity::Wrong).unwrap();
        let s = explain(&r, f);
        assert!(
            s.iter()
                .any(|s| s.model == "in-the-list" && s.explanation.contains("line 3")),
            "{:?}",
            s
        );
    }
}
