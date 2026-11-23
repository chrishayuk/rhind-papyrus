//! Does the scribe take the route rhind takes?
//!
//! Two questions, asked of the columns the papyrus writes in full.
//!
//! `routes`: rhind works the same multiplication or division its own way
//! (`scribe::multiply`, `scribe::reckon`) and compares the rows: the same
//! multipliers, in the same order, with the same ticks. Where they differ,
//! the difference is classified. rhind's way can take any part of a number
//! directly, so this asks whether the scribe's route is the one rhind would
//! have chosen, not whether rhind's is one the scribe could have written.
//!
//! `derive`: is every row of the scribe's column made from a row already
//! written, by one of a fixed list of operations the papyrus shows (`OPS`)?
//! And is every row used, ticked or the source of a later one? This needs no
//! target and makes no choice between routes, so it applies to every column
//! that begins with a row for 1, not only those `routes` can compare.
//!
//! Both read a multiplier the scribe wrote without its part sign (`5` for
//! 5̅) as the part, where the check has found that, and say so: the
//! transcription keeps the figure as written.

use crate::check::{self, Column, Run};
use crate::papyrus;
use crate::rat::Rat;
use crate::scribe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    /// row for row the same, by doubling alone
    Same,
    /// the same, multiplying the tens by a row for 10
    SameTens,
    /// the same, taking 3̅ straight from the first row
    SameThird,
    /// the same, as a division: the scribe's column is rhind's reckoning
    SameReckon,
    /// the same rows, ticked differently
    Ticks,
    /// the scribe multiplies by 10 where rhind doubles
    Tens,
    /// the scribe takes ⅔ (or 3̅) where rhind halves, or the reverse
    Thirds,
    /// rhind's rows are all among the scribe's, in order, with the same
    /// ticks; the scribe writes others besides
    Extra,
    /// another route
    Other,
}

impl Verdict {
    pub fn matches(self) -> bool {
        matches!(
            self,
            Verdict::Same | Verdict::SameTens | Verdict::SameThird | Verdict::SameReckon
        )
    }
}

/// A multiplier read otherwise than written, and why.
#[derive(Debug, Clone)]
pub struct Reading {
    pub line: usize,
    pub written: Rat,
    pub read: Rat,
    pub why: String,
}

pub struct Route {
    pub file: &'static str,
    pub line: usize,
    pub verdict: Verdict,
    pub scribe: Vec<Rat>,
    pub rhind: Vec<Rat>,
    pub readings: Vec<Reading>,
}

type Rows = Vec<(Rat, Rat, bool)>;

fn lefts(rows: &[(Rat, Rat, bool)]) -> Vec<Rat> {
    rows.iter().map(|r| r.0).collect()
}

/// The column's rows with each multiplier read as the check reads it: a
/// whole number written for a part (`5` for 5̅), where the check found the
/// part sign left off. Its product is then the base taken that part.
fn read(c: &Column, run: &Run) -> (Rows, Vec<Reading>) {
    let mut rows = c.rows.clone();
    let mut readings = Vec::new();
    for (i, row) in rows.iter_mut().enumerate() {
        let line = c.lines[i];
        if !(row.0.is_int() && row.0 > Rat::ONE) {
            continue;
        }
        let why = run
            .findings
            .iter()
            .find(|f| f.line == line && matches!(f.hint, Some(("part-sign", _))))
            .and_then(|f| f.hint.as_ref().map(|h| h.1.clone()));
        if let Some(why) = why {
            let read = row.0.recip();
            readings.push(Reading {
                line,
                written: row.0,
                read,
                why,
            });
            row.0 = read;
            if let Some(b) = c.base {
                row.1 = b * read;
            }
        }
    }
    (rows, readings)
}

/// What a column does: multiply its base by the ticked multipliers, or find
/// a number with it; and rhind's rows for the same. A division in the tens
/// style sets out the whole of its quotient with a row for 10
/// (`scribe::reckon_with`); 3̅ taken straight is a style of multiplication
/// only.
fn rhind_rows(c: &Column, rows: &Rows, style: scribe::Style, as_reckon: bool) -> Option<Rows> {
    let base = c.base?;
    if base <= Rat::ZERO || rows.is_empty() || rows[0].0 != Rat::ONE {
        return None;
    }
    let ticked_l: Rat = rows.iter().filter(|r| r.2).map(|r| r.0).sum();
    let ticked_r: Rat = rows.iter().filter(|r| r.2).map(|r| r.1).sum();
    if ticked_l.is_zero() {
        return None;
    }
    let b = scribe::express(base);
    let out = if c.op == "reckon" || as_reckon {
        let target = if c.op == "reckon" {
            c.other.unwrap_or(ticked_r)
        } else {
            ticked_r
        };
        scribe::reckon_with(&b, &scribe::express(target), style)
    } else {
        scribe::multiply_with(&b, &scribe::express(c.other.unwrap_or(ticked_l)), style)
    };
    Some(out.iter().map(|r| (r.left.value(), r.right.value(), r.tick)).collect())
}

/// Is `small` a subsequence of `big`?
fn within(small: &[Rat], big: &[Rat]) -> bool {
    let mut it = big.iter();
    small.iter().all(|x| it.any(|y| y == x))
}

fn judge(s: &[(Rat, Rat, bool)], r: &[(Rat, Rat, bool)]) -> Verdict {
    let (ls, lr) = (lefts(s), lefts(r));
    if ls == lr {
        let ts: Vec<bool> = s.iter().map(|x| x.2).collect();
        let tr: Vec<bool> = r.iter().map(|x| x.2).collect();
        return if ts == tr { Verdict::Same } else { Verdict::Ticks };
    }
    let used = |v: &[(Rat, Rat, bool)]| {
        let mut u: Vec<Rat> = v.iter().filter(|x| x.2).map(|x| x.0).collect();
        u.sort();
        u
    };
    if ls.len() > lr.len() && within(&lr, &ls) && used(s) == used(r) {
        return Verdict::Extra;
    }
    if ls.iter().any(|l| *l == Rat::int(10)) && !lr.iter().any(|l| *l == Rat::int(10)) {
        return Verdict::Tens;
    }
    let third = |v: &[Rat]| v.iter().any(|l| *l == Rat::new(2, 3) || *l == Rat::part(3));
    if third(&ls) != third(&lr) {
        return Verdict::Thirds;
    }
    Verdict::Other
}

/// How many columns the problems have, how many could be read in full, and
/// how many `routes` can compare: a first row multiplying by 1, a number it
/// multiplies, and at least one tick. The others are left out.
pub fn census() -> (usize, usize, usize) {
    let (mut all, mut clean, mut eligible) = (0, 0, 0);
    for (file, text) in papyrus::files() {
        if file.starts_with("table-") {
            continue;
        }
        let run = check::run(text);
        for c in run.columns.iter().filter(|c| c.rows.len() >= 2) {
            all += 1;
            if c.clean {
                clean += 1;
                let (rows, _) = read(c, &run);
                if rhind_rows(c, &rows, scribe::Style::default(), false).is_some() {
                    eligible += 1;
                }
            }
        }
    }
    (all, clean, eligible)
}

fn route(c: &Column, run: &Run) -> Option<(Verdict, Rows, Rows, Vec<Reading>)> {
    let (rows, readings) = read(c, run);
    let plain = rhind_rows(c, &rows, scribe::Style::default(), false)?;
    let mut verdict = judge(&rows, &plain);
    if verdict == Verdict::Same {
        return Some((verdict, rows, plain, readings));
    }
    // the conventions the papyrus uses besides doubling, one at a time, then
    // the two together
    let style = |tens, third_direct| scribe::Style { tens, third_direct };
    let tries: [(Verdict, scribe::Style, bool); 4] = [
        (Verdict::SameTens, style(true, false), false),
        (Verdict::SameThird, style(false, true), false),
        (Verdict::SameReckon, scribe::Style::default(), true),
        (Verdict::SameTens, style(true, true), false),
    ];
    for (v, st, reckon) in tries {
        if reckon && c.op == "multiply" {
            continue;
        }
        if c.op == "reckon" && st.third_direct {
            continue;
        }
        if let Some(r) = rhind_rows(c, &rows, st, reckon) {
            match judge(&rows, &r) {
                Verdict::Same => {
                    verdict = v;
                    break;
                }
                Verdict::Ticks => verdict = Verdict::Ticks,
                _ => {}
            }
        }
    }
    Some((verdict, rows, plain, readings))
}

pub fn routes() -> Vec<Route> {
    let mut out = Vec::new();
    for (file, text) in papyrus::files() {
        if file.starts_with("table-") {
            continue;
        }
        let run = check::run(text);
        for c in run.columns.iter().filter(|c| c.clean && c.rows.len() >= 2) {
            let Some((verdict, rows, plain, readings)) = route(c, &run) else {
                continue;
            };
            out.push(Route {
                file,
                line: c.first_line,
                verdict,
                scribe: lefts(&rows),
                rhind: lefts(&plain),
                readings,
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// derive: every row from a row already written

/// The operations a row may be made by, each from one row already written.
/// The list is fixed before any column is looked at; a column that needs
/// another is reported, not accommodated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    /// twice a row
    Double,
    /// half a row
    Halve,
    /// ⅔ of a row (Problem 61 and everywhere)
    TwoThirds,
    /// 3̅ of a row, without its ⅔ written (Nos. 35, 39)
    Third,
    /// 10 times the first row
    Ten,
    /// 1̅0̅ of the first row
    Tenth,
    /// n̅ of the first row, where that is 1 or a single part: 1̅5̅ of 15 is 1,
    /// 7̅ of 1 2̅ 4̅ is 4̅ (the 2 ÷ n table's working; Nos. 21, 34, 36, 70)
    UnitPart,
    /// a part named in the multiplication the page states, of the first row
    Named,
}

pub const OPS: &[(Op, &str)] = &[
    (Op::Double, "twice a row already written"),
    (Op::Halve, "half a row already written"),
    (Op::TwoThirds, "⅔ of a row already written"),
    (Op::Third, "3̅ of a row already written, its ⅔ not written"),
    (Op::Ten, "10 times the first row"),
    (Op::Tenth, "1̅0̅ of the first row"),
    (Op::UnitPart, "n̅ of the first row, where that makes 1 or a single part"),
    (Op::Named, "a part the page's statement names, of the first row"),
];

impl Op {
    pub fn name(self) -> &'static str {
        match self {
            Op::Double => "double",
            Op::Halve => "halve",
            Op::TwoThirds => "⅔",
            Op::Third => "3̅",
            Op::Ten => "×10",
            Op::Tenth => "1̅0̅",
            Op::UnitPart => "unit part",
            Op::Named => "named",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fidelity {
    /// every row made from one before it, and every row used
    Faithful,
    /// every row made from one before it; the column ticks nothing, so
    /// whether each row is used cannot be said (often a list of parts)
    NoTicks,
    /// every row made from one before it; a row neither ticked nor used
    Unused,
    /// a row no listed operation makes from the rows before it
    Underived,
}

/// For each row: its multiplier, whether ticked, and how it is made.
pub type Derived = Vec<(Rat, bool, Option<(Op, usize)>)>;

pub struct Derivation {
    pub file: &'static str,
    pub line: usize,
    /// one of the columns `routes` compares
    pub compared: bool,
    /// its `routes` verdict, if compared
    pub verdict: Option<Verdict>,
    /// if compared: rhind's own route for the column, derived the same way
    pub rhind: Option<Fidelity>,
    pub fidelity: Fidelity,
    /// for each row after the first: its multiplier, and how it is made
    /// (operation and the row it is made from), if it can be
    pub rows: Derived,
    /// rows neither ticked nor the source of a later row
    pub unused: Vec<usize>,
    pub readings: Vec<Reading>,
}

/// The ways `m` can be made from row `j` (multiplier `s`) of a column whose
/// first row multiplies `base`.
fn makes(m: Rat, j: usize, s: Rat, base: Option<Rat>, named: &[Rat]) -> Vec<Op> {
    let mut ops = Vec::new();
    if m == s * Rat::int(2) {
        ops.push(Op::Double);
    }
    if m == s / Rat::int(2) {
        ops.push(Op::Halve);
    }
    if m == s * Rat::new(2, 3) {
        ops.push(Op::TwoThirds);
    }
    if m == s / Rat::int(3) {
        ops.push(Op::Third);
    }
    if j == 0 {
        if m == Rat::int(10) {
            ops.push(Op::Ten);
        }
        if m == Rat::part(10) {
            ops.push(Op::Tenth);
        }
        if m.numer() == 1 && m.denom() > 1 {
            if let Some(b) = base.filter(|b| *b > Rat::ZERO) {
                if (b * m).numer() == 1 {
                    ops.push(Op::UnitPart);
                }
            }
        }
        if named.contains(&m) {
            ops.push(Op::Named);
        }
    }
    ops
}

/// The parts a stated multiplier names: the terms of `Multiply b by m`.
fn named_parts(c: &Column) -> Vec<Rat> {
    if c.op != "multiply" {
        return vec![];
    }
    match c.other {
        Some(m) => scribe::try_express(m)
            .map(|e| e.terms.iter().map(|t| t.value()).collect())
            .unwrap_or_default(),
        None => vec![],
    }
}

fn derive_column(c: &Column, run: &Run) -> (Fidelity, Derived, Vec<usize>, Vec<Reading>) {
    let (rows, readings) = read(c, run);
    let (fidelity, out, unused) = derive_rows(c, &rows);
    (fidelity, out, unused, readings)
}

/// Derive each row of `rows` (a column of `c`, the scribe's or rhind's) from
/// the rows before it.
fn derive_rows(c: &Column, rows: &Rows) -> (Fidelity, Derived, Vec<usize>) {
    let named = named_parts(c);
    let mut out = Vec::new();
    let mut source = vec![false; rows.len()];
    let mut underived = false;
    for (i, r) in rows.iter().enumerate() {
        if i == 0 {
            out.push((r.0, r.2, None));
            continue;
        }
        // every earlier row that makes this one counts as used; the first
        // found (nearest above) is the one shown
        let mut how = None;
        for j in (0..i).rev() {
            let ops = makes(r.0, j, rows[j].0, c.base, &named);
            if let Some(op) = ops.first() {
                source[j] = true;
                if how.is_none() {
                    how = Some((*op, j));
                }
            }
        }
        if how.is_none() {
            underived = true;
        }
        out.push((r.0, r.2, how));
    }
    let unused: Vec<usize> = (1..rows.len()).filter(|&i| !rows[i].2 && !source[i]).collect();
    let any_tick = rows.iter().any(|r| r.2);
    let unused = if any_tick { unused } else { vec![] };
    let fidelity = if underived {
        Fidelity::Underived
    } else if !any_tick {
        Fidelity::NoTicks
    } else if !unused.is_empty() {
        Fidelity::Unused
    } else {
        Fidelity::Faithful
    };
    (fidelity, out, unused)
}

/// Every column of the problems that can be read in full and begins with a
/// row for 1, derived row by row.
pub fn derive() -> Vec<Derivation> {
    let mut out = Vec::new();
    for (file, text) in papyrus::files() {
        if file.starts_with("table-") {
            continue;
        }
        let run = check::run(text);
        for c in run.columns.iter().filter(|c| c.clean && c.rows.len() >= 2) {
            if c.rows[0].0 != Rat::ONE {
                continue;
            }
            let r = route(c, &run);
            let verdict = r.as_ref().map(|r| r.0);
            let rhind = r.as_ref().map(|r| derive_rows(c, &r.2).0);
            let (fidelity, rows, unused, readings) = derive_column(c, &run);
            out.push(Derivation {
                file,
                line: c.first_line,
                compared: verdict.is_some(),
                verdict,
                rhind,
                fidelity,
                rows,
                unused,
                readings,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find<'a>(all: &'a [Derivation], file: &str, line: usize) -> &'a Derivation {
        all.iter().find(|d| d.file == file && d.line + 1 == line).unwrap()
    }

    #[test]
    fn the_scribes_routes_derive() {
        let all = derive();
        // No. 36: 1̅0̅6̅ of 106 is 1, then doubled and halved
        let d = find(&all, "no-36", 10);
        assert_eq!(d.fidelity, Fidelity::Faithful);
        assert!(d.rows.iter().any(|r| matches!(r.2, Some((Op::UnitPart, 0)))));
        // No. 22: `5` read as 5̅, made by doubling 1̅0̅
        let d = find(&all, "no-22", 7);
        assert_eq!(d.readings.len(), 1);
        assert_eq!(d.rows[2].2, Some((Op::Double, 1)));
        // No. 31: 2̅ is written to make 4̅
        assert_eq!(find(&all, "no-31", 5).fidelity, Fidelity::Faithful);
    }

    #[test]
    fn reckonings_are_tried_with_tens() {
        let r = routes();
        let v = r.iter().find(|r| r.file == "no-76" && r.line + 1 == 10).unwrap();
        assert_eq!(v.verdict, Verdict::SameTens);
    }
}
