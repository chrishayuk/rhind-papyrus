//! Does the scribe take the route rhind takes?
//!
//! For every column the papyrus writes in full, rhind works the same
//! multiplication or division its own way (`scribe::multiply`,
//! `scribe::reckon`) and compares the rows: the same multipliers, in the same
//! order, with the same ticks. Where they differ, the difference is
//! classified. This tests the reconstruction of the procedure, not the
//! scribe's arithmetic: a column can be right and take another route.

use crate::check::{self, Column};
use crate::papyrus;
use crate::rat::Rat;
use crate::scribe;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    /// the scribe writes rows he does not use, past those rhind needs
    Extra,
    /// another route
    Other,
}

pub struct Route {
    pub file: &'static str,
    pub line: usize,
    pub verdict: Verdict,
    pub scribe: Vec<Rat>,
    pub rhind: Vec<Rat>,
}

fn lefts(rows: &[(Rat, Rat, bool)]) -> Vec<Rat> {
    rows.iter().map(|r| r.0).collect()
}

/// What a column does: multiply its base by the ticked multipliers, or find
/// a number with it; and rhind's rows for the same.
fn rhind_rows(c: &Column, style: scribe::Style, as_reckon: bool) -> Option<Vec<(Rat, Rat, bool)>> {
    let base = c.base?;
    if base <= Rat::ZERO || c.rows.is_empty() || c.rows[0].0 != Rat::ONE {
        return None;
    }
    let ticked_l: Rat = c.rows.iter().filter(|r| r.2).map(|r| r.0).sum();
    let ticked_r: Rat = c.rows.iter().filter(|r| r.2).map(|r| r.1).sum();
    if ticked_l.is_zero() {
        return None;
    }
    let b = scribe::express(base);
    let rows = if c.op == "reckon" || as_reckon {
        let target = if c.op == "reckon" {
            c.other.unwrap_or(ticked_r)
        } else {
            ticked_r
        };
        scribe::reckon(&b, &scribe::express(target))
    } else {
        scribe::multiply_with(&b, &scribe::express(c.other.unwrap_or(ticked_l)), style)
    };
    Some(rows.iter().map(|r| (r.left.value(), r.right.value(), r.tick)).collect())
}

fn judge(s: &[(Rat, Rat, bool)], r: &[(Rat, Rat, bool)]) -> Verdict {
    let (ls, lr) = (lefts(s), lefts(r));
    if ls == lr {
        let ts: Vec<bool> = s.iter().map(|x| x.2).collect();
        let tr: Vec<bool> = r.iter().map(|x| x.2).collect();
        return if ts == tr { Verdict::Same } else { Verdict::Ticks };
    }
    if ls.iter().any(|l| *l == Rat::int(10)) && !lr.iter().any(|l| *l == Rat::int(10)) {
        return Verdict::Tens;
    }
    let third = |v: &[Rat]| v.iter().any(|l| *l == Rat::new(2, 3) || *l == Rat::part(3));
    if third(&ls) != third(&lr) {
        return Verdict::Thirds;
    }
    if ls.len() > lr.len()
        && ls.starts_with(
            &lr[..lr.len().saturating_sub(0)]
                .iter()
                .take_while(|x| ls.contains(x))
                .cloned()
                .collect::<Vec<_>>(),
        )
    {
        let used: Vec<Rat> = s.iter().filter(|x| x.2).map(|x| x.0).collect();
        let used_r: Vec<Rat> = r.iter().filter(|x| x.2).map(|x| x.0).collect();
        if used == used_r {
            return Verdict::Extra;
        }
    }
    Verdict::Other
}

pub fn routes() -> Vec<Route> {
    let mut out = Vec::new();
    for (file, text) in papyrus::files() {
        if file.starts_with("table-") {
            continue;
        }
        let run = check::run(text);
        for c in run.columns.iter().filter(|c| c.clean && c.rows.len() >= 2) {
            let Some(plain) = rhind_rows(c, scribe::Style::default(), false) else {
                continue;
            };
            let mut verdict = judge(&c.rows, &plain);
            if verdict != Verdict::Same {
                // the conventions the papyrus uses besides doubling, one at a time
                let tries: [(Verdict, scribe::Style, bool); 3] = [
                    (
                        Verdict::SameTens,
                        scribe::Style {
                            tens: true,
                            third_direct: false,
                        },
                        false,
                    ),
                    (
                        Verdict::SameThird,
                        scribe::Style {
                            tens: false,
                            third_direct: true,
                        },
                        false,
                    ),
                    (Verdict::SameReckon, scribe::Style::default(), true),
                ];
                for (v, style, reckon) in tries {
                    if reckon && c.op == "multiply" {
                        continue;
                    }
                    if let Some(r) = rhind_rows(c, style, reckon) {
                        match judge(&c.rows, &r) {
                            Verdict::Same => {
                                verdict = v;
                                break;
                            }
                            Verdict::Ticks => verdict = Verdict::Ticks,
                            _ => {}
                        }
                    }
                }
                if verdict != Verdict::Same
                    && verdict != Verdict::SameTens
                    && verdict != Verdict::SameThird
                    && verdict != Verdict::SameReckon
                {
                    // both together
                    if let Some(r) = rhind_rows(
                        c,
                        scribe::Style {
                            tens: true,
                            third_direct: true,
                        },
                        false,
                    ) {
                        if judge(&c.rows, &r) == Verdict::Same {
                            verdict = Verdict::SameTens;
                        }
                    }
                }
            }
            out.push(Route {
                file,
                line: c.first_line,
                verdict,
                scribe: lefts(&c.rows),
                rhind: lefts(&plain),
            });
        }
    }
    out
}
