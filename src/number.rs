//! Egyptian numbers: a whole number and distinct unit fractions, with
//! two-thirds the one fraction that is not a part of one.
//!
//! A number keeps the terms in the order the scribe wrote them, so that a
//! transcription prints back as it was read. Its value is a `Rat`.

use crate::rat::Rat;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Term {
    /// ⅔, written with its own sign
    TwoThirds,
    /// the unit fraction n̅, one part in n
    Part(u64),
    /// a whole number written among the fractions: almost always a part
    /// whose sign the scribe left off (`7 ⅔ 39` for 7 ⅔ 3̅9̅). Read as written.
    Bare(u64),
}

impl Term {
    pub fn value(self) -> Rat {
        match self {
            Term::TwoThirds => Rat::new(2, 3),
            Term::Part(n) => Rat::part(n),
            Term::Bare(n) => Rat::int(n as i128),
        }
    }
}

/// A number as the scribe writes it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Eg {
    pub whole: u64,
    pub terms: Vec<Term>,
}

impl Eg {
    pub fn int(n: u64) -> Eg {
        Eg {
            whole: n,
            terms: vec![],
        }
    }
    pub fn value(&self) -> Rat {
        Rat::int(self.whole as i128) + self.terms.iter().map(|t| t.value()).sum()
    }
    pub fn is_zero(&self) -> bool {
        self.whole == 0 && self.terms.is_empty()
    }
    /// Distinct fractions, largest first, ⅔ before all: the form the papyrus
    /// uses almost everywhere.
    pub fn is_canonical(&self) -> bool {
        let mut last = Rat::int(2);
        for t in &self.terms {
            if let Term::Bare(_) = t {
                return false;
            }
            let v = t.value();
            if v >= last {
                return false;
            }
            last = v;
        }
        true
    }
    pub fn parts(&self) -> Vec<u64> {
        self.terms
            .iter()
            .filter_map(|t| match t {
                Term::Part(n) => Some(*n),
                _ => None,
            })
            .collect()
    }
    pub fn ascii(&self) -> String {
        let mut out = Vec::new();
        if self.whole > 0 || self.terms.is_empty() {
            out.push(self.whole.to_string());
        }
        for t in &self.terms {
            out.push(match t {
                Term::TwoThirds => "2/3".to_string(),
                Term::Part(n) => format!("'{}", n),
                Term::Bare(n) => n.to_string(),
            });
        }
        out.join(" ")
    }
}

/// n̅, with a combining overline on every digit.
pub fn overbar(n: u64) -> String {
    let mut s = String::new();
    for c in n.to_string().chars() {
        s.push(c);
        s.push('\u{0305}');
    }
    s
}

impl fmt::Display for Eg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        if self.whole > 0 || self.terms.is_empty() {
            write!(f, "{}", self.whole)?;
            first = false;
        }
        for t in &self.terms {
            if !first {
                write!(f, " ")?;
            }
            first = false;
            match t {
                Term::TwoThirds => write!(f, "⅔")?,
                Term::Part(n) => write!(f, "{}", overbar(*n))?,
                Term::Bare(n) => write!(f, "{}", n)?,
            }
        }
        Ok(())
    }
}

/// Units of measure. Only the hekat and the ro convert into each other
/// (a hekat is 320 ro, as Problems 80-84 and the hekat problems use it).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unit {
    Hekat,
    Ro,
    Khar,
    Cubit,
    Setat,
    Deben,
    Kite,
    Loaf,
    Pesu,
    Hinu,
    Day,
    Khet,
    Palm,
    Finger,
}

pub const UNITS: &[(&str, Unit)] = &[
    ("hekat", Unit::Hekat),
    ("ro", Unit::Ro),
    ("khar", Unit::Khar),
    ("cubit", Unit::Cubit),
    ("cubits", Unit::Cubit),
    ("setat", Unit::Setat),
    ("deben", Unit::Deben),
    ("kite", Unit::Kite),
    ("loaf", Unit::Loaf),
    ("loaves", Unit::Loaf),
    ("pesu", Unit::Pesu),
    ("hinu", Unit::Hinu),
    ("day", Unit::Day),
    ("days", Unit::Day),
    ("khet", Unit::Khet),
    ("palm", Unit::Palm),
    ("palms", Unit::Palm),
    ("finger", Unit::Finger),
    ("fingers", Unit::Finger),
];

impl Unit {
    pub fn name(self) -> &'static str {
        UNITS.iter().find(|u| u.1 == self).map(|u| u.0).unwrap_or("?")
    }
    pub fn from_word(w: &str) -> Option<Unit> {
        UNITS.iter().find(|u| u.0 == w).map(|u| u.1)
    }
}

/// A figure as it stands on the page: one or more numbers, each with a unit or
/// none (`'4 '64 hekat 1 '3 ro`), and its value in the first unit.
#[derive(Clone, Debug, PartialEq)]
pub struct Quantity {
    pub segments: Vec<(Eg, Option<Unit>)>,
    pub value: Rat,
    pub unit: Option<Unit>,
}

impl Quantity {
    pub fn plain(eg: Eg) -> Quantity {
        let value = eg.value();
        Quantity {
            segments: vec![(eg, None)],
            value,
            unit: None,
        }
    }
    pub fn of(value: Rat) -> Quantity {
        Quantity::plain(crate::scribe::express(value))
    }
    /// Is this the exact value it was made from?
    pub fn exact(&self, value: Rat) -> bool {
        self.segments[0].0.value() == value
    }
    pub fn of_in(value: Rat, unit: Option<Unit>) -> Quantity {
        let mut q = Quantity::of(value);
        q.unit = unit;
        q.segments[0].1 = unit;
        q
    }
    pub fn eg(&self) -> &Eg {
        &self.segments[0].0
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, (eg, unit)) in self.segments.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            match unit {
                Some(Unit::Pesu) => write!(f, "pesu {}", eg)?,
                Some(u) if eg.is_zero() && eg.whole == 0 && i == 0 && self.segments.len() == 1 => {
                    write!(f, "{} {}", eg, u.name())?
                }
                Some(u) => write!(f, "{} {}", eg, u.name())?,
                None => write!(f, "{}", eg)?,
            }
        }
        Ok(())
    }
}

fn vulgar(c: char) -> Option<Term> {
    Some(match c {
        '½' => Term::Part(2),
        '⅓' => Term::Part(3),
        '¼' => Term::Part(4),
        '⅕' => Term::Part(5),
        '⅙' => Term::Part(6),
        '⅛' => Term::Part(8),
        '⅒' => Term::Part(10),
        '⅔' => Term::TwoThirds,
        _ => return None,
    })
}

/// Read one term: `5`, `'15`, `1̅5̅`, `2/3`, `''3`, `⅔`. `Ok(None)` is a whole
/// number, returned in the second place.
pub fn term(tok: &str) -> Result<(Option<Term>, Option<u64>), String> {
    if tok == "2/3" || tok == "''3" || tok == "⅔" || tok == "3\u{033F}" {
        return Ok((Some(Term::TwoThirds), None));
    }
    let mut cs = tok.chars();
    if let (Some(c), None) = (cs.next(), cs.next()) {
        if let Some(t) = vulgar(c) {
            return Ok((Some(t), None));
        }
    }
    if let Some(rest) = tok.strip_prefix('\'') {
        return match rest.parse::<u64>() {
            Ok(n) if n >= 2 => Ok((Some(Term::Part(n)), None)),
            _ => Err(format!("`{}` is not a part: write 'n for one part in n", tok)),
        };
    }
    if tok.contains('\u{0305}') {
        let digits: String = tok.chars().filter(|c| *c != '\u{0305}').collect();
        return match digits.parse::<u64>() {
            Ok(n) if n >= 2 => Ok((Some(Term::Part(n)), None)),
            _ => Err(format!("`{}` is not a part", tok)),
        };
    }
    if !tok.is_empty() && tok.chars().all(|c| c.is_ascii_digit()) {
        return tok
            .parse::<u64>()
            .map(|n| (None, Some(n)))
            .map_err(|_| format!("`{}` is too large", tok));
    }
    Err(format!("`{}` is not a number", tok))
}

/// Read a figure. Marks of damage, ink and doubt (`[ ]`, `{ }`, `?`) must
/// already have been taken off.
pub fn parse(text: &str) -> Result<Quantity, String> {
    let mut segments: Vec<(Eg, Option<Unit>)> = Vec::new();
    let mut cur = Eg::default();
    let mut any = false;
    let mut pesu_next = false;
    for tok in text.split_whitespace() {
        if let Some(u) = Unit::from_word(tok) {
            if u == Unit::Pesu {
                pesu_next = true;
                continue;
            }
            if !any {
                return Err(format!("`{}` with no number before it", tok));
            }
            segments.push((std::mem::take(&mut cur), Some(u)));
            any = false;
            continue;
        }
        let (t, w) = term(tok)?;
        if let Some(w) = w {
            if !any {
                cur.whole = w;
            } else {
                // after the whole number or a fraction: written without its part sign
                cur.terms.push(Term::Bare(w));
            }
        }
        if let Some(t) = t {
            cur.terms.push(t);
        }
        any = true;
    }
    if any {
        segments.push((cur, if pesu_next { Some(Unit::Pesu) } else { None }));
    } else if pesu_next || segments.is_empty() {
        return Err("no number here".into());
    }
    let unit = segments[0].1;
    let mut value = Rat::ZERO;
    for (eg, u) in &segments {
        let v = eg.value();
        value = value
            + match (unit, u) {
                (Some(Unit::Hekat), Some(Unit::Ro)) => v / Rat::int(320),
                (Some(Unit::Ro), Some(Unit::Hekat)) => v * Rat::int(320),
                // land: a cubit-strip is a hundredth of a setat
                (Some(Unit::Setat), Some(Unit::Cubit)) => v / Rat::int(100),
                (a, b) if a == *b || b.is_none() => v,
                (_, None) => v,
                (_, Some(b)) => {
                    return Err(format!(
                        "{} and {} in one figure",
                        unit.map(|u| u.name()).unwrap_or("a number"),
                        b.name()
                    ))
                }
            };
    }
    Ok(Quantity { segments, value, unit })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_and_writes() {
        let q = parse("5 '3 '15").unwrap();
        assert_eq!(q.value, Rat::new(27, 5));
        assert_eq!(q.to_string(), "5 3̅ 1̅5̅");
        assert_eq!(parse("2/3 '10 '30").unwrap().value, Rat::new(4, 5));
        assert_eq!(parse("1̅5̅").unwrap().value, Rat::part(15));
        assert_eq!(
            parse("'4 '64 hekat 1 '3 ro").unwrap().value,
            Rat::new(17, 64) + Rat::new(4, 3 * 320)
        );
        assert_eq!(parse("7 2/3 39").unwrap().to_string(), "7 ⅔ 39");
        assert_eq!(
            parse("'2 '8 setat 7 '2 cubit").unwrap().value,
            Rat::new(5, 8) + Rat::new(15, 200)
        );
        assert!(parse("'1").is_err());
        assert!(!parse("'4 '2").unwrap().eg().is_canonical());
    }
}
