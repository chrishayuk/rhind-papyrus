//! Running a page: every row, total and result the scribe wrote is worked
//! again with the scribe's operations and compared, exactly, with what is
//! written. Working the papyrus leaves out is written in.

use crate::number::{Eg, Quantity, Unit};
use crate::page::{self, Figure, Kind, Line, Statement};
use crate::rat::Rat;
use crate::scribe::{self, Row};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// the scribe's figure is not what the working gives
    Wrong,
    /// the value is right but written otherwise than the scribe usually writes
    Form,
    /// could not be checked: lost, restored or doubtful
    Unchecked,
    /// the line is not in the notation
    Notation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    pub line: usize,
    pub span: (usize, usize),
    pub severity: Severity,
    pub message: String,
    /// the value the working gives, if the finding is about a figure
    pub expected: Option<Rat>,
    pub written: Option<Rat>,
    /// for a wrong row: is the error carried into the total?
    pub carried: Option<bool>,
    /// for a wrong sum of a list: the lines of the figures summed
    pub items: Vec<usize>,
    /// a slip seen while checking, that the figures alone do not show
    pub hint: Option<(&'static str, String)>,
}

/// What `rhind` says about one line.
#[derive(Clone, Debug, PartialEq)]
pub enum Mark {
    None,
    Right,
    Wrong(String),
    Unchecked,
}

#[derive(Clone, Debug, Default)]
pub struct Run {
    pub lines: Vec<Line>,
    pub marks: Vec<Mark>,
    pub findings: Vec<Finding>,
    /// working written in by rhind, after the given line
    pub written_in: Vec<(usize, Vec<String>)>,
    pub sections: Vec<Section>,
    /// every column the page writes, as checked
    pub columns: Vec<Column>,
}

/// A column as the scribe wrote it: what it multiplies, and its rows.
#[derive(Clone, Debug)]
pub struct Column {
    pub first_line: usize,
    pub base: Option<Rat>,
    /// `Multiply`, `Reckon`, or neither
    pub op: &'static str,
    /// the statement's other number: the multiplier, or the number to find
    pub other: Option<Rat>,
    /// multiplier, product, ticked: as written
    pub rows: Vec<(Rat, Rat, bool)>,
    /// every row could be read, none restored or in a table's double statement
    pub clean: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Section {
    pub heading: String,
    pub line: usize,
    pub checked: usize,
    pub right: usize,
    pub wrong: usize,
    pub unchecked: usize,
}

fn val(f: &Figure) -> Option<Rat> {
    if f.lost {
        return None;
    }
    f.value.as_ref().map(|q| q.value)
}

fn unit(f: &Figure) -> Option<Unit> {
    f.value.as_ref().and_then(|q| q.unit)
}

/// Show a value as the scribe would write it, with its unit.
pub fn show(v: Rat, u: Option<Unit>) -> String {
    if v < Rat::ZERO {
        return format!("-{}", show(-v, u));
    }
    let q = Quantity::of_in(v, u);
    q.to_string()
}

/// A figure is checkable if it is there and not in doubt.
fn checkable(f: &Figure) -> bool {
    !f.lost && !f.uncertain && f.value.is_some()
}

struct ColRow {
    line: usize,
    tick: bool,
    left: Figure,
    right: Figure,
    /// the table's double statement, `4  28  4̅`: 4 times n is 28, and 2̅8̅ of n is 4̅
    third: Option<Figure>,
}

#[derive(Clone, Copy, PartialEq)]
enum Op {
    Multiply,
    Divide,
    Unknown,
}

/// What the last column, list or sentence gave, for a Total or Result after it.
#[derive(Clone)]
struct Pending {
    /// for `Total`: as written, and as it should be
    total: Option<(Rat, Rat)>,
    /// for `Result`
    result: Option<(Rat, Rat)>,
    unit: Option<Unit>,
    /// lines of rows found wrong, to say whether the error is carried
    wrong_rows: Vec<usize>,
    other: Option<(Rat, Rat)>,
    /// the column's rows: line, product as it should be, ticked
    rows: Vec<(usize, Rat, bool)>,
}

impl Run {
    fn mark(&mut self, line: usize, m: Mark) {
        // a wrong mark is never overwritten by a right one
        if matches!(self.marks[line], Mark::Wrong(_)) && m == Mark::Right {
            return;
        }
        self.marks[line] = m;
    }
    fn find(&mut self, line: usize, span: (usize, usize), severity: Severity, message: String) -> usize {
        self.findings.push(Finding {
            line,
            span,
            severity,
            message,
            expected: None,
            written: None,
            carried: None,
            items: vec![],
            hint: None,
        });
        self.findings.len() - 1
    }
    fn section(&mut self) -> Option<&mut Section> {
        self.sections.last_mut()
    }
    fn count(&mut self, ok: Option<bool>) {
        if let Some(s) = self.section() {
            match ok {
                Some(true) => {
                    s.checked += 1;
                    s.right += 1
                }
                Some(false) => {
                    s.checked += 1;
                    s.wrong += 1
                }
                None => s.unchecked += 1,
            }
        }
    }
}

/// Convert a value between hekat and ro; other units only to themselves.
fn convert(v: Rat, from: Option<Unit>, to: Option<Unit>) -> Rat {
    match (from, to) {
        (Some(Unit::Ro), Some(Unit::Hekat)) => v / Rat::int(320),
        (Some(Unit::Hekat), Some(Unit::Ro)) => v * Rat::int(320),
        _ => v,
    }
}

fn check_column(
    run: &mut Run,
    col: &[ColRow],
    stmt: Option<(Op, Option<Figure>, Option<Figure>)>,
    section_base: Option<&Figure>,
) -> Pending {
    let (op, base_fig, other_fig) = stmt.unwrap_or((Op::Unknown, None, None));
    let base_fig = base_fig.or_else(|| section_base.cloned());
    // the number being multiplied: from the statement, or the row whose
    // multiplier is 1, or the first row
    let stated: Option<(Rat, Option<Unit>)> = base_fig
        .as_ref()
        .filter(|f| checkable(f))
        .map(|f| (val(f).unwrap(), unit(f)));
    let mut base = stated;
    let mut first_defines = false;
    // which number does the column multiply? Candidates: the statement's,
    // the row for 1 as written, that row with a part sign put back, and the
    // number each other row implies. The one that makes most rows right is
    // taken, so that one slip in the row for 1 does not make every row look
    // wrong (decision `outvote`). On a tie the earlier candidate wins.
    let rows: Vec<(Rat, Rat)> = col
        .iter()
        .filter(|r| checkable(&r.left) && checkable(&r.right) && r.third.is_none())
        .map(|r| (val(&r.left).unwrap(), val(&r.right).unwrap()))
        .collect();
    let one = col
        .iter()
        .find(|r| checkable(&r.left) && val(&r.left) == Some(Rat::ONE) && checkable(&r.right));
    let mut cands: Vec<(Rat, Option<Unit>)> = Vec::new();
    if let Some(b) = stated {
        cands.push(b);
    }
    if let Some(r) = one {
        let u = unit(&r.right);
        cands.push((val(&r.right).unwrap(), u));
        if let Some(q) = r.right.value.as_ref() {
            for v in crate::slips::with_part_signs(q.eg()) {
                cands.push((v, u));
            }
        }
    }
    let implied: Vec<(Rat, Option<Unit>)> = col
        .iter()
        .filter(|r| checkable(&r.left) && checkable(&r.right) && r.third.is_none())
        .filter(|r| !val(&r.left).unwrap().is_zero())
        .map(|r| (val(&r.right).unwrap() / val(&r.left).unwrap(), unit(&r.right)))
        .collect();
    if stated.is_none() && one.is_some() {
        cands.extend(implied.iter().cloned());
    }
    // a number the page states is what the column multiplies
    if stated.is_some() {
        cands.truncate(1);
    }
    cands.retain(|c| !c.0.is_zero());
    let score = |b: Rat| rows.iter().filter(|(l, w)| *l * b == *w).count();
    if let Some(best) = cands
        .iter()
        .copied()
        .max_by(|a, b| score(a.0).cmp(&score(b.0)).then(std::cmp::Ordering::Greater))
    {
        // max_by keeps the last of equals; prefer the first
        let top = score(best.0);
        base = cands.iter().copied().find(|c| score(c.0) == top);
    }
    if base.is_none() {
        if let Some(r) = col.first().filter(|r| checkable(&r.left) && checkable(&r.right)) {
            let l = val(&r.left).unwrap();
            if !l.is_zero() {
                base = Some((val(&r.right).unwrap() / l, unit(&r.right)));
                first_defines = true;
            }
        }
    }
    let mut written_l = Rat::ZERO;
    let mut written_r = Rat::ZERO;
    let mut true_r = Rat::ZERO;
    let mut true_l = Rat::ZERO;
    let mut all_r_written = Rat::ZERO;
    let mut all_r_true = Rat::ZERO;
    let any_tick = col.iter().any(|r| r.tick);
    let mut wrong_rows = Vec::new();
    let mut complete = true;
    let mut prev_true: Option<Rat> = None;
    for (i, r) in col.iter().enumerate() {
        let ok_figs = checkable(&r.left) && checkable(&r.right);
        if !ok_figs {
            complete = false;
            run.mark(r.line, Mark::Unchecked);
            run.count(None);
            let which = if !checkable(&r.left) { &r.left } else { &r.right };
            let why = if which.lost {
                "lost"
            } else if which.uncertain {
                "doubtful"
            } else {
                "not a number"
            };
            run.find(
                r.line,
                which.span,
                Severity::Unchecked,
                format!("not checked: the figure is {}", why),
            );
            continue;
        }
        let l = val(&r.left).unwrap();
        if let (Some(third), Some((b, _))) = (r.third.as_ref(), base) {
            // a × n is k, and k̅ of n is c: the part sign on k is optional
            let k = r.right.value.as_ref().map(|q| {
                let e = q.eg();
                if e.whole == 0 && e.terms.len() == 1 {
                    Rat::int(e.parts()[0] as i128)
                } else {
                    q.value
                }
            });
            if !checkable(third) || k.is_none() {
                run.mark(r.line, Mark::Unchecked);
                run.count(None);
                continue;
            }
            let k = k.unwrap();
            let c = val(third).unwrap();
            let restored = r.left.restored || r.right.restored || third.restored;
            let mut ok = true;
            for (fig, want, got, what) in [(&r.right, l * b, k, "times"), (third, b / k, c, "part")] {
                if want != got {
                    ok = false;
                    let msg = if what == "times" {
                        format!(
                            "{} times {} is {}; the papyrus has {}",
                            r.left.value.as_ref().unwrap(),
                            show(b, None),
                            show(want, None),
                            fig.text
                        )
                    } else {
                        format!(
                            "{} of {} is {}; the papyrus has {}",
                            crate::number::overbar(k.floor() as u64),
                            show(b, None),
                            show(want, None),
                            fig.value.as_ref().map(|q| q.to_string()).unwrap_or_default()
                        )
                    };
                    if restored {
                        run.find(
                            r.line,
                            fig.span,
                            Severity::Unchecked,
                            format!("the edition's restoration disagrees with the working: {}", msg),
                        );
                    } else {
                        let f = run.find(r.line, fig.span, Severity::Wrong, msg);
                        run.findings[f].expected = Some(want);
                        run.findings[f].written = Some(got);
                        wrong_rows.push(f);
                    }
                }
            }
            if restored {
                run.mark(r.line, Mark::Unchecked);
                run.count(None);
            } else if ok {
                run.mark(r.line, Mark::Right);
                run.count(Some(true));
            } else {
                run.mark(
                    r.line,
                    Mark::Wrong(format!("{}  {}", show(l * b, None), show(b / (l * b), None))),
                );
                run.count(Some(false));
            }
            if r.tick {
                written_l = written_l + Rat::ONE / (l * b);
                true_l = true_l + Rat::ONE / (l * b);
                written_r = written_r + c;
                true_r = true_r + b / (l * b);
            }
            all_r_written = all_r_written + c;
            all_r_true = all_r_true + b / (l * b);
            prev_true = None;
            continue;
        }
        // in the table, `2  3̅4̅` says 2 times 17 is 34, with the part sign
        // of the part about to be taken
        if let (Some(_), Some((b, _)), Some(q)) = (section_base, base, r.right.value.as_ref()) {
            let e = q.eg();
            if l.is_int()
                && e.whole == 0
                && e.terms.len() == 1
                && e.parts().len() == 1
                && Rat::int(e.parts()[0] as i128) == l * b
            {
                run.mark(r.line, Mark::Right);
                run.count(Some(true));
                prev_true = None;
                continue;
            }
        }
        let w = convert(val(&r.right).unwrap(), unit(&r.right), base.and_then(|b| b.1));
        let mut expected = match base {
            Some((b, _)) if !(first_defines && i == 0) => Some(l * b),
            _ => None,
        };
        // "add its half": a row of fractions only may be taken of the row
        // above rather than of the first row (decision `relative`)
        let mut relative = false;
        if let (Some(e), Some(p)) = (expected, prev_true) {
            if e != w && l < Rat::ONE && l * p == w {
                expected = Some(w);
                relative = true;
            }
        }
        let t = expected.unwrap_or(w);
        prev_true = Some(t);
        if r.tick {
            written_l = written_l + l;
            written_r = written_r + w;
            true_l = true_l + l;
            true_r = true_r + t;
        }
        all_r_written = all_r_written + w;
        all_r_true = all_r_true + t;
        let restored = r.left.restored || r.right.restored;
        match expected {
            Some(e) if e != w && restored => {
                run.mark(r.line, Mark::Unchecked);
                run.count(None);
                run.find(
                    r.line,
                    r.right.span,
                    Severity::Unchecked,
                    format!(
                        "the edition's restoration disagrees with the working: {} of {} is {}",
                        r.left.value.as_ref().map(|q| q.to_string()).unwrap_or_default(),
                        show(base.unwrap().0, base.and_then(|b| b.1)),
                        show(e, base.and_then(|b| b.1))
                    ),
                );
            }
            Some(_) if restored => {
                run.mark(r.line, Mark::Unchecked);
                run.count(None);
            }
            Some(e) if e != w => {
                let u = base.and_then(|b| b.1);
                let msg = format!(
                    "{} of {} is {}; the papyrus has {}",
                    r.left.value.as_ref().map(|q| q.to_string()).unwrap_or_default(),
                    show(base.unwrap().0, u),
                    show(e, u),
                    r.right.value.as_ref().map(|q| q.to_string()).unwrap_or_default()
                );
                let k = run.find(r.line, r.right.span, Severity::Wrong, msg);
                run.findings[k].expected = Some(e);
                run.findings[k].written = Some(w);
                if l.is_int() && l > Rat::ONE && base.unwrap().0 / l == w {
                    let n = l.floor() as u64;
                    run.findings[k].span = r.left.span;
                    run.findings[k].message = format!(
                        "{} of {} is {}: the multiplier is written {} for {}",
                        crate::number::overbar(n),
                        show(base.unwrap().0, u),
                        r.right.value.as_ref().unwrap(),
                        n,
                        crate::number::overbar(n)
                    );
                    run.findings[k].hint = Some((
                        "part-sign",
                        format!(
                            "{} written for {}: the part sign left off the multiplier",
                            n,
                            crate::number::overbar(n)
                        ),
                    ));
                    run.findings[k].expected = None;
                } else if let Some(why) = (!w.is_zero() && !base.unwrap().0.is_zero())
                    .then(|| crate::slips::multiplier_slip(r.left.value.as_ref().unwrap().eg(), w / base.unwrap().0))
                    .flatten()
                {
                    run.findings[k].span = r.left.span;
                    run.findings[k].hint = Some(("multiplier", why));
                } else if l.is_int() && l > Rat::ONE {
                    // the part sign left off both the multiplier and the product
                    let n = l.floor() as u64;
                    let alt = crate::scribe::express(base.unwrap().0 / l);
                    if let Some(w_eg) = r.right.value.as_ref().map(|q| q.eg().clone()) {
                        if let Some(why) = crate::slips::part_sign(&alt, &w_eg) {
                            run.findings[k].span = r.left.span;
                            run.findings[k].message = format!(
                                "{} of {} is {}: the papyrus has {} and {}",
                                crate::number::overbar(n),
                                show(base.unwrap().0, u),
                                alt,
                                n,
                                w_eg
                            );
                            run.findings[k].hint = Some((
                                "part-sign",
                                format!(
                                    "{} written for {}, and in the product {}",
                                    n,
                                    crate::number::overbar(n),
                                    why
                                ),
                            ));
                            run.findings[k].expected = None;
                        }
                    }
                }
                run.mark(r.line, Mark::Wrong(show(e, u)));
                run.count(Some(false));
                wrong_rows.push(k);
            }
            Some(_) => {
                run.mark(r.line, Mark::Right);
                run.count(Some(true));
                if relative {
                    run.find(
                        r.line,
                        r.left.span,
                        Severity::Form,
                        "taken of the row above, not of the first row: \"add its part\"".into(),
                    );
                }
                if r.left.restored || r.right.restored {
                    // right, but only because of a restoration
                }
                if let Some(eg) = r.right.value.as_ref().map(|q| q.eg()) {
                    if !eg.is_canonical() {
                        run.find(
                            r.line,
                            r.right.span,
                            Severity::Form,
                            "the value is right; its parts are not written largest first, or one repeats".into(),
                        );
                    }
                }
            }
            None => {}
        }
    }
    let u = base.and_then(|b| b.1);
    run.columns.push(Column {
        first_line: col.first().map(|r| r.line).unwrap_or(0),
        base: base.map(|b| b.0),
        op: match op {
            Op::Multiply => "multiply",
            Op::Divide => "reckon",
            Op::Unknown => "",
        },
        other: other_fig.as_ref().filter(|f| checkable(f)).and_then(val),
        rows: col
            .iter()
            .filter(|r| checkable(&r.left) && checkable(&r.right))
            .map(|r| (val(&r.left).unwrap(), val(&r.right).unwrap(), r.tick))
            .collect(),
        clean: col.iter().all(|r| {
            checkable(&r.left) && checkable(&r.right) && !r.left.restored && !r.right.restored && r.third.is_none()
        }),
    });
    // the ticks against the statement
    if complete && any_tick {
        match op {
            Op::Multiply => {
                if let Some(m) = other_fig.as_ref().filter(|f| checkable(f)) {
                    let m = val(m).unwrap();
                    if written_l != m {
                        let line = col.last().unwrap().line;
                        run.find(
                            line,
                            (0, 0),
                            Severity::Wrong,
                            format!(
                                "the ticked rows multiply by {}, not {}",
                                show(written_l, None),
                                show(m, None)
                            ),
                        );
                    }
                }
            }
            Op::Divide => {
                if let Some(target) = other_fig.as_ref().filter(|f| checkable(f)) {
                    let tv = val(target).unwrap();
                    if true_r != tv {
                        let line = col.last().unwrap().line;
                        run.find(
                            line,
                            (0, 0),
                            Severity::Wrong,
                            format!(
                                "the ticked rows make {}, not {}",
                                show(true_r, u),
                                show(tv, unit(target))
                            ),
                        );
                    }
                }
            }
            Op::Unknown => {}
        }
    }
    let (tw, tt) = if any_tick {
        (written_r, true_r)
    } else {
        (all_r_written, all_r_true)
    };
    let result = match op {
        Op::Divide => Some((written_l, true_l)),
        Op::Multiply => Some((written_r, true_r)),
        Op::Unknown => None,
    };
    Pending {
        total: if complete { Some((tw, tt)) } else { None },
        result: if complete { result } else { None },
        unit: u,
        wrong_rows,
        other: if complete && any_tick && op == Op::Unknown {
            Some((written_l, true_l))
        } else {
            None
        },
        rows: col
            .iter()
            .filter(|r| checkable(&r.left) && checkable(&r.right) && r.third.is_none())
            .map(|r| {
                (
                    r.line,
                    base.map(|b| val(&r.left).unwrap() * b.0)
                        .unwrap_or(val(&r.right).unwrap()),
                    r.tick,
                )
            })
            .collect(),
    }
}

/// Check a `Total` or `Result` against what came before it.
fn check_claim(run: &mut Run, line: usize, f: &Figure, what: &str, pending: &Option<Pending>, result: bool) {
    let Some(p) = pending else {
        return;
    };
    let candidates: Vec<(Rat, Rat)> = if result {
        p.result
            .iter()
            .chain(p.other.iter())
            .chain(p.total.iter())
            .cloned()
            .collect()
    } else {
        p.total.iter().cloned().collect()
    };
    if candidates.is_empty() {
        return;
    }
    if !checkable(f) {
        run.mark(line, Mark::Unchecked);
        run.count(None);
        return;
    }
    let v = convert(val(f).unwrap(), unit(f), p.unit);
    let hit = candidates.iter().find(|(_, t)| *t == v);
    let carried_hit = candidates.iter().find(|(w, _)| *w == v);
    if hit.is_some() {
        run.mark(line, Mark::Right);
        run.count(Some(true));
        for &k in &p.wrong_rows {
            run.findings[k].carried = Some(false);
        }
    } else {
        let (_, t) = candidates[0];
        let carried = carried_hit.is_some();
        let msg = if carried {
            format!(
                "the {} is {}; the papyrus has {}, carrying the error above",
                what,
                show(t, p.unit),
                f.value.as_ref().unwrap()
            )
        } else {
            format!(
                "the {} is {}; the papyrus has {}",
                what,
                show(t, p.unit),
                f.value.as_ref().unwrap()
            )
        };
        let k = run.find(line, f.span, Severity::Wrong, msg);
        run.findings[k].expected = Some(t);
        run.findings[k].written = Some(v);
        if !result && p.rows.len() >= 2 && p.rows.len() <= 14 {
            // the fewest changes of tick that make the written total
            let n = p.rows.len();
            let mut best: Option<(usize, u32)> = None;
            for mask in 1u32..(1 << n) {
                let sum: Rat = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| p.rows[i].1).sum();
                if sum == v {
                    let changes = (0..n).filter(|i| (mask & (1 << i) != 0) != p.rows[*i].2).count();
                    if best.map(|b| changes < b.0).unwrap_or(true) {
                        best = Some((changes, mask));
                    }
                }
            }
            if let Some((changes, mask)) = best {
                let name = |i: usize| format!("line {}", p.rows[i].0 + 1);
                let add: Vec<String> = (0..n)
                    .filter(|i| mask & (1 << i) != 0 && !p.rows[*i].2)
                    .map(name)
                    .collect();
                let drop: Vec<String> = (0..n)
                    .filter(|i| mask & (1 << i) == 0 && p.rows[*i].2)
                    .map(name)
                    .collect();
                let mut why = format!(
                    "the total is right for the rows on {}",
                    (0..n)
                        .filter(|i| mask & (1 << i) != 0)
                        .map(name)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                if !add.is_empty() {
                    why.push_str(&format!("; a tick left off {}", add.join(", ")));
                }
                if !drop.is_empty() {
                    why.push_str(&format!("; a tick wrongly on {}", drop.join(", ")));
                }
                if changes > 0 {
                    run.findings[k].hint = Some(("ticks", why));
                }
            }
        }
        run.mark(line, Mark::Wrong(show(t, p.unit)));
        run.count(Some(false));
        for &j in &p.wrong_rows {
            run.findings[j].carried = Some(carried);
        }
    }
}

fn write_rows(rows: &[Row]) -> Vec<String> {
    let w = rows
        .iter()
        .map(|r| r.left.to_string().chars().count())
        .max()
        .unwrap_or(1);
    rows.iter()
        .map(|r| {
            let l = r.left.to_string();
            let pad = w - l.chars().count();
            format!(
                "{} {}{}   {}",
                if r.tick { "\\" } else { " " },
                " ".repeat(pad),
                l,
                r.right
            )
        })
        .collect()
}

fn eg_of(f: &Figure) -> Option<Eg> {
    f.value
        .as_ref()
        .filter(|q| q.segments.len() == 1)
        .map(|q| q.eg().clone())
}

pub fn run(text: &str) -> Run {
    let lines = page::read(text);
    let n = lines.len();
    let mut run = Run {
        marks: vec![Mark::None; n],
        ..Default::default()
    };
    let mut pending: Option<Pending> = None;
    let mut stmt: Option<(usize, Op, Option<Figure>, Option<Figure>)> = None;
    let mut col: Vec<ColRow> = Vec::new();
    let mut list: Vec<(usize, Figure)> = Vec::new();
    let mut list_lines: Vec<usize> = Vec::new();
    // in an entry of the 2 ÷ n table, every column multiplies n
    let mut section_base: Option<Figure> = None;
    let mut section_num: Option<Rat> = None;
    // the products of the red parts at the head of a table entry
    let mut section_heads: Vec<Rat> = Vec::new();

    let mut i = 0;
    while i <= n {
        let line = lines.get(i);
        let is_row = matches!(line.map(|l| &l.kind), Some(Kind::Row { .. }));
        let is_blank = matches!(line.map(|l| &l.kind), Some(Kind::Blank));
        let is_fig = matches!(line.map(|l| &l.kind), Some(Kind::Figure(_)));
        // a column ends at the first line that is not a row, at a blank
        // line, or where a row multiplies by 1 again: the papyrus sets small
        // columns side by side, and each begins with its row for 1
        let starts_again = match line.map(|l| &l.kind) {
            Some(Kind::Row { cells, .. }) => {
                !col.is_empty()
                    && cells
                        .first()
                        .and_then(|c| c.value.as_ref())
                        .map(|q| q.value == Rat::ONE)
                        .unwrap_or(false)
            }
            _ => false,
        };
        if (!is_row || starts_again) && !col.is_empty() {
            let s = stmt.take().map(|(_, op, a, b)| (op, a, b));
            pending = Some(check_column(&mut run, &col, s, section_base.as_ref()));
            col.clear();
        }
        if !is_fig && !is_blank && !list.is_empty() {
            let w: Option<Rat> = list.iter().map(|(_, f)| if checkable(f) { val(f) } else { None }).sum();
            list_lines = list.iter().map(|(l, _)| *l).collect();
            pending = w.map(|w| Pending {
                total: Some((w, w)),
                result: None,
                unit: list.iter().find_map(|(_, f)| unit(f)),
                wrong_rows: vec![],
                other: None,
                rows: vec![],
            });
            list.clear();
        }
        // a statement with no column after it: write the working in
        if let Some((at, op, a, b)) = stmt.clone() {
            if !is_row && !is_blank {
                if let (Some(a), Some(b)) = (a.as_ref().and_then(eg_of), b.as_ref().and_then(eg_of)) {
                    let rows = match op {
                        Op::Multiply => scribe::multiply(&a, &b),
                        _ => scribe::reckon(&a, &b),
                    };
                    let (tl, tr) = scribe::total(&rows);
                    let mut out = write_rows(&rows);
                    if op == Op::Multiply {
                        out.push(format!("Total {}.", scribe::express(tr)));
                    }
                    run.written_in.push((at, out));
                    let res = if op == Op::Multiply { tr } else { tl };
                    pending = Some(Pending {
                        total: Some((tr, tr)),
                        result: Some((res, res)),
                        unit: None,
                        wrong_rows: vec![],
                        other: None,
                        rows: vec![],
                    });
                }
                stmt = None;
            }
        }
        let Some(line) = line else { break };
        match &line.kind {
            Kind::Heading(h) => {
                run.sections.push(Section {
                    heading: h.clone(),
                    line: i,
                    ..Default::default()
                });
                pending = None;
                stmt = None;
                section_base = None;
                section_num = None;
                section_heads.clear();
                if let Some((a, d)) = h.split_once(" ÷ ") {
                    if a != "No." {
                        // an entry of a table: reckon with d to find a
                        let fa = page::figure(a, 0);
                        let fd = page::figure(d, 0);
                        section_num = val(&fa);
                        section_base = Some(fd);
                    }
                }
            }
            Kind::Statement(s) => match s {
                Statement::Multiply(a, b) => stmt = Some((i, Op::Multiply, Some(a.clone()), Some(b.clone()))),
                Statement::Reckon(a, b) => stmt = Some((i, Op::Divide, Some(a.clone()), Some(b.clone()))),
                Statement::Total(f) => {
                    let is_list = pending
                        .as_ref()
                        .map(|p| p.result.is_none() && p.other.is_none())
                        .unwrap_or(false)
                        && !list_lines.is_empty();
                    check_claim(
                        &mut run,
                        i,
                        f,
                        if is_list { "sum of the figures above" } else { "total" },
                        &pending,
                        false,
                    );
                    if is_list {
                        if let Some(k) = run
                            .findings
                            .iter()
                            .rposition(|x| x.line == i && x.severity == Severity::Wrong)
                        {
                            run.findings[k].items = list_lines.clone();
                        }
                    }
                    list_lines.clear();
                }
                Statement::Result(f) => check_claim(&mut run, i, f, "result", &pending, true),
                Statement::Remainder(f) => {
                    let mut p = pending.clone();
                    if let Some(n) = section_num {
                        // in the table: what is left of the 2 once the
                        // parts at the head of the entry are taken, one, two
                        // or more of them
                        let mut cands: Vec<Rat> = Vec::new();
                        let mut acc = Rat::ZERO;
                        for v in &section_heads {
                            acc = acc + *v;
                            cands.push(n - acc);
                        }
                        if let Some(pp) = p.as_ref().and_then(|p| p.total) {
                            cands.push(n - pp.1);
                        }
                        let v = if checkable(f) { val(f) } else { None };
                        let pick = cands.iter().copied().find(|c| Some(*c) == v).or(cands.first().copied());
                        p = pick.map(|c| Pending {
                            total: None,
                            result: Some((c, c)),
                            unit: None,
                            wrong_rows: vec![],
                            other: None,
                            rows: vec![],
                        });
                    }
                    check_claim(&mut run, i, f, "remainder", &p, true)
                }
                Statement::Add(a, b) | Statement::Take(a, b) => {
                    if let (Some(x), Some(y)) = (val(a).filter(|_| checkable(a)), val(b).filter(|_| checkable(b))) {
                        let v = if matches!(s, Statement::Add(..)) {
                            x + convert(y, unit(b), unit(a))
                        } else {
                            convert(y, unit(b), unit(a)) - x
                        };
                        pending = Some(Pending {
                            total: Some((v, v)),
                            result: Some((v, v)),
                            unit: unit(a).or(unit(b)),
                            wrong_rows: vec![],
                            other: None,
                            rows: vec![],
                        });
                    }
                }
                Statement::Proof => {}
            },
            Kind::Row { tick, cells, .. } => {
                if cells.len() >= 2 {
                    let third = (cells.len() == 3 && section_base.is_some()).then(|| cells[2].clone());
                    if section_base.is_some() && cells[0].red {
                        if let Some(v) = val(&cells[1]) {
                            section_heads.push(v);
                        }
                    }
                    col.push(ColRow {
                        line: i,
                        tick: *tick,
                        left: cells[0].clone(),
                        right: cells[1].clone(),
                        third,
                    });
                }
            }
            Kind::Figure(f) => list.push((i, f.clone())),
            Kind::Bad(e) => {
                let len = line.text.chars().count();
                run.find(i, (0, len), Severity::Notation, e.clone());
            }
            Kind::Words(w) => {
                // the divisor written alone at the head of a table entry
                let t = w.trim();
                if let (Some(fd), true) = (
                    section_base.as_ref(),
                    !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()),
                ) {
                    let written: Rat = Rat::int(t.parse::<i128>().unwrap_or(0));
                    if let Some(n) = val(fd) {
                        let col = line.text.find(t).map(|b| line.text[..b].chars().count()).unwrap_or(0);
                        let span = (col, col + t.chars().count());
                        if written == n {
                            run.mark(i, Mark::Right);
                            run.count(Some(true));
                        } else {
                            let k = run.find(
                                i,
                                span,
                                Severity::Wrong,
                                format!("the divisor of this entry is {}; the papyrus has {}", n, t),
                            );
                            run.findings[k].expected = Some(n);
                            run.findings[k].written = Some(written);
                            run.mark(i, Mark::Wrong(n.to_string()));
                            run.count(Some(false));
                        }
                    }
                }
            }
            Kind::Blank => {}
        }
        i += 1;
    }
    run.lines = lines;
    run
}

/// The gutter column for marks.
const GUTTER: usize = 40;

pub fn egyptian(s: &str) -> String {
    // turn 'n into n̅ and 2/3 into ⅔ outside quotes and notes
    let mut out = String::new();
    let mut quoted = false;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '%' && !quoted {
            out.extend(&chars[i..]);
            break;
        }
        if c == '"' {
            quoted = !quoted;
        }
        if !quoted && c == '\'' && chars.get(i + 1).map(|d| d.is_ascii_digit()).unwrap_or(false) {
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                out.push(chars[i]);
                out.push('\u{0305}');
                i += 1;
            }
            continue;
        }
        if !quoted && chars[i..].starts_with(&['2', '/', '3']) {
            out.push('⅔');
            i += 3;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

fn width(s: &str) -> usize {
    s.chars().filter(|c| *c != '\u{0305}').count()
}

/// The page as `rhind` prints it: the lines in the scribe's figures, with a
/// mark in the margin for every line checked, and the working written in.
pub fn render(run: &Run, marks: bool) -> String {
    let mut out = String::new();
    for (i, line) in run.lines.iter().enumerate() {
        let text = egyptian(line.text.trim_end());
        out.push_str(&text);
        if marks {
            let pad = GUTTER.saturating_sub(width(&text)).max(2);
            match &run.marks[i] {
                Mark::Right => out.push_str(&format!("{}✓", " ".repeat(pad))),
                Mark::Wrong(e) => out.push_str(&format!("{}✗ {}", " ".repeat(pad), e)),
                Mark::Unchecked => out.push_str(&format!("{}?", " ".repeat(pad))),
                Mark::None => {}
            }
        }
        out.push('\n');
        for (at, rows) in &run.written_in {
            if *at == i {
                for r in rows {
                    out.push_str(r);
                    if marks {
                        let pad = GUTTER.saturating_sub(width(r)).max(2);
                        out.push_str(&format!("{}· rhind", " ".repeat(pad)));
                    }
                    out.push('\n');
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const P27: &str = "No. 27
\"A quantity whose fifth part is added to it becomes 21.\"
  1   5
 '5   1
Total 6.
\\ 1   6
\\ 2   12
\\ '2  3
Total 21.
\\ 1   3 '2
  2   7
\\ 4   15     % Peet: (sic)
\"The quantity is 17 '2\"
";
    #[test]
    fn problem_27_finds_the_fifteen() {
        let r = run(P27);
        let wrong: Vec<_> = r.findings.iter().filter(|f| f.severity == Severity::Wrong).collect();
        assert_eq!(wrong.len(), 1, "{:#?}", r.findings);
        assert_eq!(wrong[0].line, 11);
        assert_eq!(wrong[0].expected, Some(Rat::int(14)));
        assert_eq!(r.sections[0].wrong, 1);
    }
    #[test]
    fn carried_or_not() {
        let r = run("\\ 1   3 '2\n  2   7\n\\ 4   15\nTotal 17 '2.\n");
        let f = r.findings.iter().find(|f| f.severity == Severity::Wrong).unwrap();
        assert_eq!(f.carried, Some(false));
        let r = run("\\ 1   3 '2\n  2   7\n\\ 4   15\nTotal 18 '2.\n");
        let f = r.findings.iter().find(|f| f.line == 2).unwrap();
        assert_eq!(f.carried, Some(true));
    }
    #[test]
    fn writes_in_working() {
        let r = run("Multiply 9 by 9.\nResult 81.\n");
        assert_eq!(r.written_in.len(), 1);
        assert!(r.findings.is_empty(), "{:#?}", r.findings);
        assert_eq!(r.marks[1], Mark::Right);
    }
    #[test]
    fn table_entry() {
        let r = run("2 ÷ 7\n\\ '4   1 '2 '4\n\\ '28  '4\n");
        assert!(r.findings.is_empty(), "{:#?}", r.findings);
        assert_eq!(r.sections[0].right, 2);
    }
}
