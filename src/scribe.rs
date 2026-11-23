//! The scribe's operations, done the scribe's way.
//!
//! Nothing here divides. A multiplication is a column of doublings with the
//! rows wanted ticked; a division is a multiplication run backwards, doubling
//! the divisor until the rows can be picked to make the dividend. Fractions
//! are halved, doubled (an odd part through the 2 ÷ n table), taken
//! two-thirds of, or taken a tenth of. Where the papyrus does not say how a
//! choice was made, the choice made here is listed in `DECISIONS` and named
//! where it is made.

use crate::number::{Eg, Quantity, Term, Unit};
use crate::rat::Rat;
use crate::table;

/// Write a value as the scribe would: the whole part, then ⅔ if it fits,
/// then the 2 ÷ n table if what is left is twice a part, then the largest
/// part that fits, again and again (decision `express`).
pub fn express(v: Rat) -> Eg {
    assert!(v >= Rat::ZERO, "the scribe has no negative numbers");
    let whole = v.floor() as u64;
    let f = v.fract();
    let plain = fraction(f, false);
    let mut eg = if f >= Rat::new(2, 3) {
        // with ⅔ or without: the fewer terms, and on a tie the one whose
        // smallest part is largest (3/4 is 2̅ 4̅, not ⅔ 1̅2̅)
        let with = fraction(f, true);
        let key = |t: &Vec<Term>| {
            (
                t.len(),
                t.iter()
                    .map(|x| match x {
                        Term::Part(n) => *n,
                        _ => 0,
                    })
                    .max()
                    .unwrap_or(0),
            )
        };
        if key(&with) < key(&plain) {
            with
        } else {
            plain
        }
    } else {
        plain
    };
    let mut out = Eg {
        whole,
        terms: std::mem::take(&mut eg),
    };
    sort(&mut out);
    out
}

/// The shortest sum of distinct unit fractions for `f`, each part at most
/// `dmax`, with at most `k` terms; on a tie, the one whose smallest part is
/// largest. `None` if there is none within the bounds.
fn search(f: Rat, k: usize, min_d: u64, dmax: u64) -> Option<Vec<u64>> {
    if f.is_zero() {
        return Some(vec![]);
    }
    if k == 0 {
        return None;
    }
    if f.numer() == 1 && (f.denom() as u64) >= min_d && (f.denom() as u64) <= dmax {
        return Some(vec![f.denom() as u64]);
    }
    if k == 1 {
        return None;
    }
    // the first part lies between the largest that fits and k times that
    let lo = (((f.denom() + f.numer() - 1) / f.numer()) as u64).max(min_d);
    let hi = ((k as i128 * f.denom() / f.numer()) as u64).min(dmax);
    let mut best: Option<Vec<u64>> = None;
    for d in lo..=hi {
        let rest = f - Rat::part(d);
        if rest < Rat::ZERO {
            continue;
        }
        let limit = best.as_ref().map(|b| b.len() - 1).unwrap_or(k - 1);
        if let Some(mut tail) = search(
            rest,
            limit,
            d + 1,
            best.as_ref().map(|b| *b.last().unwrap()).unwrap_or(dmax).min(dmax),
        ) {
            tail.insert(0, d);
            let better = match &best {
                None => true,
                Some(b) => (tail.len(), *tail.last().unwrap()) < (b.len(), *b.last().unwrap()),
            };
            if better {
                best = Some(tail);
            }
        }
    }
    best
}

fn fraction(f: Rat, two_thirds: bool) -> Vec<Term> {
    // two parts of an odd number: the papyrus's own table, whatever is shorter
    if !two_thirds && f.numer() == 2 {
        if let Some(parts) = table::twice(f.denom() as u64) {
            return parts.iter().map(|d| Term::Part(*d)).collect();
        }
    }
    let greedy = greedy(f, two_thirds);
    // look for a shorter or smaller form than the greedy one, within its bounds
    let lead = if two_thirds && f >= Rat::new(2, 3) {
        Rat::new(2, 3)
    } else {
        Rat::ZERO
    };
    let rest = f - lead;
    let parts: Vec<u64> = greedy
        .iter()
        .filter_map(|t| if let Term::Part(n) = t { Some(*n) } else { None })
        .collect();
    if rest.denom() <= 2000 && !parts.is_empty() && parts.len() <= 4 {
        let dmax = *parts.iter().max().unwrap();
        if let Some(found) = search(rest, parts.len(), 2, dmax) {
            let mut out: Vec<Term> = if lead.is_zero() { vec![] } else { vec![Term::TwoThirds] };
            out.extend(found.into_iter().map(Term::Part));
            return out;
        }
    }
    greedy
}

fn greedy(f: Rat, two_thirds: bool) -> Vec<Term> {
    let mut f = f;
    let mut terms = Vec::new();
    if two_thirds && f >= Rat::new(2, 3) {
        terms.push(Term::TwoThirds);
        f = f - Rat::new(2, 3);
    }
    let mut guard = 0;
    while !f.is_zero() && guard < 12 {
        guard += 1;
        if f.numer() == 1 {
            terms.push(Term::Part(f.denom() as u64));
            break;
        }
        if f.numer() == 2 {
            if let Some(parts) = table::twice(f.denom() as u64) {
                terms.extend(parts.iter().map(|d| Term::Part(*d)));
                break;
            }
        }
        // the largest part that fits; past a part in a thousand million the
        // scribe would never go, and the arithmetic here would overflow
        if f.denom() > 1_000_000_000_000 {
            break;
        }
        let d = (f.denom() + f.numer() - 1) / f.numer();
        terms.push(Term::Part(d as u64));
        f = f - Rat::part(d as u64);
    }
    terms
}

fn sort(eg: &mut Eg) {
    eg.terms.sort_by(|a, b| b.value().cmp(&a.value()));
}

/// Put a sum of terms right: if two are the same, or they no longer go
/// largest first, write the value afresh.
fn tidy(whole: u64, terms: Vec<Term>) -> Eg {
    let mut eg = Eg { whole, terms };
    let frac: Rat = eg.terms.iter().map(|t| t.value()).sum();
    if frac >= Rat::ONE {
        return express(eg.value());
    }
    sort(&mut eg);
    let mut seen = std::collections::HashSet::new();
    if eg.terms.iter().all(|t| seen.insert(*t)) {
        eg
    } else {
        express(eg.value())
    }
}

/// Twice a number, term by term: an even part halves its denominator, an odd
/// one goes through the table, ⅔ becomes 1 3̅.
pub fn double(x: &Eg) -> Eg {
    let mut whole = x.whole * 2;
    let mut terms = Vec::new();
    for t in &x.terms {
        match *t {
            Term::TwoThirds => {
                whole += 1;
                terms.push(Term::Part(3));
            }
            Term::Bare(n) => whole += 2 * n,
            Term::Part(2) => whole += 1,
            Term::Part(3) => terms.push(Term::TwoThirds),
            Term::Part(n) if n % 2 == 0 => terms.push(Term::Part(n / 2)),
            Term::Part(n) => match table::twice(n) {
                Some(parts) => terms.extend(parts.iter().map(|d| Term::Part(*d))),
                None => terms.extend(express(Rat::new(2, n as i128)).terms),
            },
        }
    }
    tidy(whole, terms)
}

/// Half a number, term by term: an odd whole leaves 2̅, n̅ becomes 2n̅, ⅔
/// becomes 3̅.
pub fn halve(x: &Eg) -> Eg {
    let mut terms = Vec::new();
    if x.whole % 2 == 1 {
        terms.push(Term::Part(2));
    }
    for t in &x.terms {
        terms.push(match *t {
            Term::TwoThirds => Term::Part(3),
            Term::Part(n) => Term::Part(2 * n),
            Term::Bare(_) => return express(x.value() / Rat::int(2)),
        });
    }
    tidy(x.whole / 2, terms)
}

/// Two-thirds of a number. Of a part: an odd n̅ gives 2n̅ 6n̅, the rule the
/// papyrus states in Problem 61B; an even one, 3n/2 with a bar. Of ⅔, 3̅ 9̅
/// (Problem 61). Of a whole number, by value.
pub fn two_thirds(x: &Eg) -> Eg {
    let mut terms = Vec::new();
    let mut rest = Rat::ZERO;
    if x.whole.is_multiple_of(3) {
        rest = Rat::int((x.whole / 3 * 2) as i128);
    } else {
        rest = rest + Rat::new(2 * x.whole as i128, 3);
    }
    for t in &x.terms {
        match *t {
            Term::TwoThirds => terms.extend([Term::Part(3), Term::Part(9)]),
            Term::Part(n) if n % 2 == 1 => terms.extend([Term::Part(2 * n), Term::Part(6 * n)]),
            Term::Part(n) => terms.push(Term::Part(3 * n / 2)),
            Term::Bare(_) => return express(x.value() * Rat::new(2, 3)),
        }
    }
    let base = express(rest);
    let mut all = base.terms;
    all.extend(terms);
    tidy(base.whole, all)
}

/// A row of a column: the multiplier and the product, ticked or not.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub tick: bool,
    pub left: Eg,
    pub right: Eg,
}

/// One part in `n` of a number, by value.
fn part_of(x: &Eg, n: u64) -> Eg {
    express(x.value() / Rat::int(n as i128))
}

/// The row for a fraction of the multiplicand, made from the rows already
/// written where it can be: half of the row above, two-thirds of the first
/// row, half of the ⅔ row for 3̅, a tenth by value.
fn fraction_row(rows: &[Row], base: &Eg, t: Term) -> Row {
    let one = rows[0].right.clone();
    let _ = base;
    let right = match t {
        Term::TwoThirds => two_thirds(&one),
        Term::Part(2) => halve(&one),
        Term::Part(n) => {
            // halve a row already there, if one is twice this part
            let want = Rat::part(n);
            match rows.iter().find(|r| r.left.value() == want * Rat::int(2)) {
                Some(r) => halve(&r.right),
                None => part_of(&one, n),
            }
        }
        Term::Bare(n) => express(one.value() * Rat::int(n as i128)),
    };
    let left = Eg {
        whole: 0,
        terms: vec![t],
    };
    Row {
        tick: false,
        left,
        right,
    }
}

/// Ways of setting out a multiplication that the papyrus uses, beyond
/// doubling. `rhind routes` tests which of them the scribe's columns follow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Style {
    /// tens by a row for 10 (and its doublings), units by doubling 1
    pub tens: bool,
    /// 3̅ written straight from the first row, without a ⅔ row before it
    pub third_direct: bool,
}

/// Multiply `base` by `by`: the column a scribe writes (decision `multiply`).
pub fn multiply(base: &Eg, by: &Eg) -> Vec<Row> {
    multiply_with(base, by, Style::default())
}

pub fn multiply_with(base: &Eg, by: &Eg, style: Style) -> Vec<Row> {
    let mut rows = vec![Row {
        tick: false,
        left: Eg::int(1),
        right: base.clone(),
    }];
    if style.tens && by.whole >= 10 {
        // 1, 10, 20, 40 … for the tens, then 2, 4, 8 from the first row for the units
        let tens = by.whole / 10;
        let units = by.whole % 10;
        let ten = express(base.value() * Rat::int(10));
        rows.push(Row {
            tick: false,
            left: Eg::int(10),
            right: ten,
        });
        let mut p = 10u64;
        while p * 2 <= tens * 10 {
            let next = double(&rows.last().unwrap().right);
            p *= 2;
            rows.push(Row {
                tick: false,
                left: Eg::int(p),
                right: next,
            });
        }
        let mut w = tens * 10;
        for r in rows.iter_mut().skip(1).rev() {
            if r.left.whole <= w {
                r.tick = true;
                w -= r.left.whole;
            }
        }
        let mut q = 1u64;
        let mut unit_rows = Vec::new();
        let mut prev = base.clone();
        while q * 2 <= units {
            prev = double(&prev);
            q *= 2;
            unit_rows.push(Row {
                tick: false,
                left: Eg::int(q),
                right: prev.clone(),
            });
        }
        let mut u = units;
        if u % 2 == 1 {
            rows[0].tick = true;
            u -= 1;
        }
        for r in unit_rows.iter_mut().rev() {
            if r.left.whole <= u {
                r.tick = true;
                u -= r.left.whole;
            }
        }
        rows.extend(unit_rows);
    } else {
        let mut p = 1u64;
        while p * 2 <= by.whole {
            let next = double(&rows.last().unwrap().right);
            p *= 2;
            rows.push(Row {
                tick: false,
                left: Eg::int(p),
                right: next,
            });
        }
        // tick the doublings that make the whole multiplier
        let mut w = by.whole;
        for r in rows.iter_mut().rev() {
            if r.left.whole <= w {
                r.tick = true;
                w -= r.left.whole;
            }
        }
        if by.whole == 0 {
            rows[0].tick = false;
        }
    }
    // then the fractions, ⅔ before 3̅ so that 3̅ can be half of it
    let mut fracs = by.terms.clone();
    fracs.sort_by(|a, b| b.value().cmp(&a.value()));
    for t in fracs {
        if t == Term::Part(3) && !style.third_direct && !rows.iter().any(|r| r.left.terms == [Term::TwoThirds]) {
            let r = fraction_row(&rows, base, Term::TwoThirds);
            rows.push(r);
        }
        let mut r = fraction_row(&rows, base, t);
        if t == Term::Part(3) {
            if let Some(tt) = rows.iter().find(|r| r.left.terms == [Term::TwoThirds]) {
                r.right = halve(&tt.right);
            }
        }
        r.tick = true;
        rows.push(r);
    }
    rows
}

/// The product of a column: the ticked rows added.
pub fn total(rows: &[Row]) -> (Rat, Rat) {
    rows.iter()
        .filter(|r| r.tick)
        .fold((Rat::ZERO, Rat::ZERO), |(l, r), row| {
            (l + row.left.value(), r + row.right.value())
        })
}

/// "Reckon with `by` to find `target`": double `by` until the rows can be
/// picked to make `target`; what is left over is made with halves, or with
/// ⅔ and its halves, or with tenths, whichever needs fewest rows; failing
/// all three, with parts of `by` itself (decision `reckon`).
pub fn reckon(by: &Eg, target: &Eg) -> Vec<Row> {
    let d = by.value();
    let t = target.value();
    let mut rows = vec![Row {
        tick: false,
        left: Eg::int(1),
        right: by.clone(),
    }];
    let mut p = 1u64;
    while d * Rat::int((p * 2) as i128) <= t {
        let next = double(&rows.last().unwrap().right);
        p *= 2;
        rows.push(Row {
            tick: false,
            left: Eg::int(p),
            right: next,
        });
    }
    let mut got = Rat::ZERO;
    for r in rows.iter_mut().rev() {
        let v = r.right.value();
        if got + v <= t {
            r.tick = true;
            got = got + v;
        }
    }
    let left = t - got;
    if left.is_zero() {
        return rows;
    }
    // three ways to go on with fractions of `by`
    let chains: [&[Term]; 3] = [
        &[
            Term::Part(2),
            Term::Part(4),
            Term::Part(8),
            Term::Part(16),
            Term::Part(32),
            Term::Part(64),
        ],
        &[
            Term::TwoThirds,
            Term::Part(3),
            Term::Part(6),
            Term::Part(12),
            Term::Part(24),
        ],
        &[Term::Part(10), Term::Part(5), Term::Part(20)],
    ];
    let mut best: Option<Vec<Row>> = None;
    for chain in chains {
        let mut extra: Vec<Row> = Vec::new();
        let mut need = left;
        for &term in chain {
            if need.is_zero() {
                break;
            }
            let mut all = rows.clone();
            all.extend(extra.iter().cloned());
            let mut r = fraction_row(&all, by, term);
            if term == Term::Part(3) {
                if let Some(tt) = all.iter().find(|r| r.left.terms == [Term::TwoThirds]) {
                    r.right = halve(&tt.right);
                }
            }
            let v = r.right.value();
            if v <= need {
                r.tick = true;
                need = need - v;
            }
            extra.push(r);
        }
        if need.is_zero() {
            // drop rows after the last ticked one
            while extra.last().map(|r| !r.tick).unwrap_or(false) {
                extra.pop();
            }
            if best.as_ref().map(|b| extra.len() < b.len()).unwrap_or(true) {
                best = Some(extra);
            }
        }
    }
    match best {
        Some(extra) => rows.extend(extra),
        None => {
            // parts of the divisor, one row for each, as the 2 ÷ n table
            // sets them out: 4̅ of 7 is 1 2̅ 4̅, 2̅8̅ of 7 is 4̅
            let q = express(left / d);
            for t in q.terms {
                let part = Eg {
                    whole: 0,
                    terms: vec![t],
                };
                rows.push(Row {
                    tick: true,
                    right: express(part.value() * d),
                    left: part,
                });
            }
        }
    }
    rows
}

/// What a column gives: for a multiplication the ticked products, for a
/// division the ticked multipliers.
pub fn answer(rows: &[Row], division: bool) -> Eg {
    let (l, r) = total(rows);
    express(if division { l } else { r })
}

pub fn unit_of(q: &Quantity) -> Option<Unit> {
    q.unit
}

/// The choices this engine makes that the papyrus does not fix.
pub const DECISIONS: &[(&str, &str)] = &[
    (
        "express",
        "A value is written as its whole part and then its fraction: the fewest distinct unit fractions that make it, with ⅔ allowed first, and among those the one whose smallest part is largest, so that 3/4 is 2̅ 4̅ (not ⅔ 1̅2̅) and 7/8 is 2̅ 4̅ 8̅ (not 2̅ 3̅ 2̅4̅). Twice an odd part uses the 2 ÷ n table. rhind compares values, not forms, so this rule only decides how rhind writes a figure, never whether the scribe's is right. The rule is ours.",
    ),
    (
        "double",
        "Twice a number is made term by term: n̅ for even n becomes (n/2)̅; for odd n the 2 ÷ n table is used, as the scribe would have; ⅔ becomes 1 3̅; 2̅ becomes 1. Repeated terms are then rewritten by value. The term-by-term part is the papyrus's (the table exists for it); the rewriting is ours.",
    ),
    (
        "two-thirds",
        "⅔ of an odd part n̅ is 2n̅ 6n̅, the rule stated in Problem 61B; of ⅔ it is 3̅ 9̅, as in Problem 61. ⅔ of a whole number not divisible by 3 is written by value. 3̅ of a number is half of its ⅔, as the papyrus finds it everywhere it shows the step.",
    ),
    (
        "multiply",
        "A multiplication doubles the first row until the next doubling would pass the whole multiplier, ticks the doublings that make it, then adds one row for each fraction of the multiplier, ⅔ before 3̅. The papyrus also multiplies by 10 directly and by other routes; rhind checks any route the scribe wrote, but writes only this one.",
    ),
    (
        "reckon",
        "A division doubles the divisor while the double does not pass the dividend, ticks the largest rows that fit, then makes the remainder with the halving chain (2̅ 4̅ 8̅ …), the ⅔ chain (⅔ 3̅ 6̅ …) or tenths (1̅0̅ 5̅), whichever finishes in fewest rows; failing those, with parts of the divisor itself, n̅ of the divisor being 1. The order of preference is ours.",
    ),
    (
        "columns",
        "A column ends at a blank line, at a line that is not a row, or at a row whose multiplier is 1 when the column already has rows: the papyrus sets small columns side by side, and each starts with its row for 1. In an entry of the 2 ÷ n table every column multiplies the divisor. This is a reading of the layout, ours.",
    ),
    (
        "relative",
        "In a column the page gives no statement for, a row of fractions only (2̅, 1̅0̅, ⅔) is right if it is that part of the first row, or that part of the row just above it as written: the papyrus writes `10  640` then `2̅  320` for \"add its half\" (No. 41). rhind says which it took. Where the page states the multiplication, this is not allowed: the row must be that part of the number stated (independent review, finding 8). Ours.",
    ),
    (
        "base",
        "A column multiplies the number the page states, if it states one. Otherwise the row for 1 as written, or that row with a part sign put back on one of its numbers (9 for 9̅): whichever makes more of the other rows right, the written one on a tie; the row for 1 does not vote for itself, and no other row can propose a number. A column with no row for 1 is read the same way from its first row. If there is a choice and no candidate makes any other row right, the column is ambiguous and is not checked; with no choice, the row for 1 is what the page says the column multiplies. This replaces a vote in which any row could propose the number, which put the error on correct figures where the scribe's own error had been carried down a column (Nos. 11, 12, 18; independent review, finding 3). Ours.",
    ),
    (
        "table-rows",
        "In the 2 ÷ n table a row `a  k  c` is the scribe's double statement: a times n is k, and k̅ of n is c. The part sign on k is optional, as the papyrus writes it both ways; a row `a  k̅` says the same without c. c is checked against 1/a, so one wrong k is one finding, not two (independent review, finding 10). A `Remainder` in an entry is what is left of 2 once the parts at the head of the entry, one or more, are taken. Read from Chace's notes on 2 ÷ 7 and 2 ÷ 17.",
    ),
    (
        "partial",
        "A division may stop short of the number to find: the ticked rows make less, the page writes the remainder, and the division is finished in further steps (No. 70). Falling short is not reported; a `Remainder` is checked against what is left; the result the page then gives is checked only if it is the result of a column rhind has, and otherwise reported as not checked. A multiplication or division with nothing ticked does not say which rows make the answer, so its result is not checked.",
    ),
    (
        "units",
        "Figures are compared, and added in lists, only in units that agree: a bare number goes with anything, hekat and ro convert (320 ro to the hekat), and any other pair is not checked. Inside one figure, setat and cubit-strips convert (100 to the setat); a cubit on its own is a length and is not converted.",
    ),
    (
        "bare",
        "A whole number standing among fractions (`7 ⅔ 39`) is read as written, a whole number, and its value counts as such. The papyrus usually means a part whose sign was left off; rhind finds that by checking, and names it, rather than by correcting the figure. Chace (vol. II, p. xi) says such an omission, where the context makes it clear, need not be an error; rhind reports these as notation, apart from other differences.",
    ),
    (
        "restored",
        "A figure an edition restores ([…]) is used to work out what follows, but a row, total or result containing one is never counted right or wrong, and a list with a restored member is not summed: it tests the edition, not the scribe. Where a restoration disagrees with the working, rhind says so.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::parse;
    fn eg(s: &str) -> Eg {
        parse(s).unwrap().segments[0].0.clone()
    }
    #[test]
    fn doubling_uses_the_table() {
        assert_eq!(double(&eg("'7")).to_string(), "4̅ 2̅8̅");
        assert_eq!(double(&eg("3 '2")).to_string(), "7");
        assert_eq!(double(&eg("2/3")).to_string(), "1 3̅");
        assert_eq!(double(&eg("'3 '15")).to_string(), "⅔ 1̅0̅ 3̅0̅");
    }
    #[test]
    fn halving_and_two_thirds() {
        assert_eq!(halve(&eg("7")).to_string(), "3 2̅");
        assert_eq!(two_thirds(&eg("'5")).to_string(), "1̅0̅ 3̅0̅");
        assert_eq!(two_thirds(&eg("2/3")).to_string(), "3̅ 9̅");
        assert_eq!(two_thirds(&eg("9")).to_string(), "6");
    }
    #[test]
    fn problem_26() {
        // reckon with 5 to find 15
        let rows = reckon(&eg("5"), &eg("15"));
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.tick));
        assert_eq!(answer(&rows, true).to_string(), "3");
        // multiply 3 by 4
        let rows = multiply(&eg("3"), &eg("4"));
        assert_eq!(rows.iter().filter(|r| r.tick).count(), 1);
        assert_eq!(answer(&rows, false).to_string(), "12");
    }
    #[test]
    fn problem_27() {
        // 21 ÷ 6: 1 6, 2 12 ticked, then a half, 3, ticked
        let rows = reckon(&eg("6"), &eg("21"));
        assert_eq!(answer(&rows, true).to_string(), "3 2̅");
        let rows = multiply(&eg("3 '2"), &eg("5"));
        assert_eq!(rows[2].right.to_string(), "14");
        assert_eq!(answer(&rows, false).to_string(), "17 2̅");
    }
    #[test]
    fn remainders_as_parts_of_the_divisor() {
        let rows = reckon(&eg("7"), &eg("2"));
        assert_eq!(answer(&rows, true).to_string(), "4̅ 2̅8̅");
    }
}
