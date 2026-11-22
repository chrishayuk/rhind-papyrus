//! Exact fractions. The scribe's arithmetic is exact, so the check of it is
//! too: every figure in the papyrus is turned into a fraction and compared
//! exactly. These are ours, not the scribe's; they only judge.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rat {
    n: i128,
    d: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Rat {
    pub const ZERO: Rat = Rat { n: 0, d: 1 };
    pub const ONE: Rat = Rat { n: 1, d: 1 };

    pub fn new(n: i128, d: i128) -> Rat {
        assert!(d != 0, "a fraction with nothing under it");
        let g = gcd(n, d).max(1);
        let s = if d < 0 { -1 } else { 1 };
        Rat {
            n: s * n / g,
            d: s * d / g,
        }
    }
    pub fn int(n: i128) -> Rat {
        Rat { n, d: 1 }
    }
    /// One part in `d`: the Egyptian unit fraction d̅.
    pub fn part(d: u64) -> Rat {
        Rat::new(1, d as i128)
    }
    pub fn numer(self) -> i128 {
        self.n
    }
    pub fn denom(self) -> i128 {
        self.d
    }
    pub fn is_int(self) -> bool {
        self.d == 1
    }
    pub fn is_zero(self) -> bool {
        self.n == 0
    }
    pub fn floor(self) -> i128 {
        self.n.div_euclid(self.d)
    }
    pub fn fract(self) -> Rat {
        self - Rat::int(self.floor())
    }
    pub fn recip(self) -> Rat {
        Rat::new(self.d, self.n)
    }
    pub fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

impl Add for Rat {
    type Output = Rat;
    fn add(self, o: Rat) -> Rat {
        let g = gcd(self.d, o.d);
        Rat::new(self.n * (o.d / g) + o.n * (self.d / g), self.d / g * o.d)
    }
}
impl Sub for Rat {
    type Output = Rat;
    fn sub(self, o: Rat) -> Rat {
        self + (-o)
    }
}
impl Neg for Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        Rat { n: -self.n, d: self.d }
    }
}
impl Mul for Rat {
    type Output = Rat;
    fn mul(self, o: Rat) -> Rat {
        let g1 = gcd(self.n, o.d).max(1);
        let g2 = gcd(o.n, self.d).max(1);
        Rat::new((self.n / g1) * (o.n / g2), (self.d / g2) * (o.d / g1))
    }
}
impl Div for Rat {
    type Output = Rat;
    fn div(self, o: Rat) -> Rat {
        self * o.recip()
    }
}
impl Ord for Rat {
    fn cmp(&self, o: &Rat) -> Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
}
impl PartialOrd for Rat {
    fn partial_cmp(&self, o: &Rat) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl std::iter::Sum for Rat {
    fn sum<I: Iterator<Item = Rat>>(it: I) -> Rat {
        it.fold(Rat::ZERO, |a, b| a + b)
    }
}
impl fmt::Debug for Rat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}
impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic() {
        assert_eq!(Rat::part(3) + Rat::part(15), Rat::new(2, 5));
        assert_eq!(Rat::new(7, 2) * Rat::int(4), Rat::int(14));
        assert_eq!(Rat::int(21) / Rat::int(6), Rat::new(7, 2));
        assert!(Rat::new(2, 3) > Rat::part(2));
        assert_eq!(Rat::new(7, 2).floor(), 3);
    }
}
