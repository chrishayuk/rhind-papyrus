//! How did the scribe choose the entries of the 2 ÷ n table?
//!
//! Each rule here is a way of choosing a sum of unit fractions for 2/n. Each
//! is run for every odd n from 5 to 101 and compared with the papyrus. A rule
//! that reproduces an entry is evidence that the entry could have been made
//! that way, not that it was. The rules are named for what they do; none is
//! claimed to be any particular scholar's.

use crate::rat::Rat;
use crate::table;

pub struct Rule {
    pub name: &'static str,
    pub what: &'static str,
    pub choose: fn(u64) -> Option<Vec<u64>>,
}

/// The divisors of q².
fn divisors_of_square(q: i128) -> Vec<i128> {
    let mut f: Vec<(i128, u32)> = Vec::new();
    let mut n = q;
    let mut p = 2i128;
    while p * p <= n {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        if e > 0 {
            f.push((p, 2 * e));
        }
        p += 1;
    }
    if n > 1 {
        f.push((n, 2));
    }
    let mut out = vec![1i128];
    for (p, e) in f {
        let mut next = Vec::new();
        for d in &out {
            let mut m = *d;
            for _ in 0..=e {
                next.push(m);
                m *= p;
            }
        }
        out = next;
    }
    out
}

/// Every sum of at most `k` distinct unit fractions, each part between
/// `min_d` and `dmax`, that makes `f`.
fn all(f: Rat, k: usize, min_d: u64, dmax: u64, out: &mut Vec<Vec<u64>>, cur: &mut Vec<u64>) {
    if f.is_zero() {
        out.push(cur.clone());
        return;
    }
    if k == 0 || f < Rat::ZERO {
        return;
    }
    if f.numer() == 1 && f.denom() as u64 >= min_d && f.denom() as u64 <= dmax {
        cur.push(f.denom() as u64);
        out.push(cur.clone());
        cur.pop();
    }
    if k == 2 {
        // p/q = 1/c + 1/d exactly when (pc - q)(pd - q) = q²
        let (p, q) = (f.numer(), f.denom());
        let qq = q * q;
        for x in divisors_of_square(q) {
            if x * x >= qq {
                continue;
            }
            if (q + x) % p == 0 && (q + qq / x) % p == 0 {
                let c = ((q + qq / x) / p) as u64;
                let d = ((q + x) / p) as u64;
                let (c, d) = (c.min(d), c.max(d));
                if c >= min_d && d <= dmax && c < d {
                    cur.extend([c, d]);
                    out.push(cur.clone());
                    cur.truncate(cur.len() - 2);
                }
            }
        }
        return;
    }
    let lo = (((f.denom() + f.numer() - 1) / f.numer()) as u64).max(min_d);
    let hi = ((k as i128 * f.denom() / f.numer()).min(dmax as i128)) as u64;
    for d in lo..=hi {
        let rest = f - Rat::part(d);
        if rest.is_zero() {
            continue; // counted above
        }
        cur.push(d);
        all(rest, k - 1, d + 1, dmax, out, cur);
        cur.pop();
    }
}

fn candidates(n: u64) -> Vec<Vec<u64>> {
    thread_local! {
        static CACHE: std::cell::RefCell<std::collections::HashMap<u64, Vec<Vec<u64>>>> = Default::default();
    }
    CACHE.with(|c| {
        c.borrow_mut()
            .entry(n)
            .or_insert_with(|| {
                let mut out = Vec::new();
                all(Rat::new(2, n as i128), 4, 2, 1000, &mut out, &mut Vec::new());
                out.sort();
                out.dedup();
                out
            })
            .clone()
    })
}

fn greedy(n: u64) -> Option<Vec<u64>> {
    let mut f = Rat::new(2, n as i128);
    let mut v = Vec::new();
    while !f.is_zero() && v.len() < 8 {
        let d = (f.denom() + f.numer() - 1) / f.numer();
        v.push(d as u64);
        f = f - Rat::part(d as u64);
    }
    Some(v)
}

fn shortest(n: u64) -> Option<Vec<u64>> {
    candidates(n).into_iter().min_by_key(|c| (c.len(), *c.last().unwrap()))
}

fn shortest_even(n: u64) -> Option<Vec<u64>> {
    candidates(n)
        .into_iter()
        .min_by_key(|c| (c.len(), c.iter().filter(|d| *d % 2 == 1).count(), *c.last().unwrap()))
}

fn smallest_last(n: u64) -> Option<Vec<u64>> {
    candidates(n).into_iter().min_by_key(|c| (*c.last().unwrap(), c.len()))
}

/// 2/n = 1/m + (2m - n)/(mn), with 2m - n written as distinct divisors of m:
/// each divisor d gives the part m·n/d. This is what the scribe's red
/// auxiliary numbers in many entries look like.
fn divisor_sums(n: u64) -> Vec<Vec<u64>> {
    let mut out = Vec::new();
    for m in (n / 2 + 1)..=n {
        let r = 2 * m - n;
        let divs: Vec<u64> = (1..=m).filter(|d| m % d == 0).rev().collect();
        // subsets of the divisors adding to r, at most three
        let mut subsets: Vec<Vec<u64>> = Vec::new();
        fn pick(divs: &[u64], r: u64, k: usize, cur: &mut Vec<u64>, out: &mut Vec<Vec<u64>>) {
            if r == 0 {
                out.push(cur.clone());
                return;
            }
            if k == 0 {
                return;
            }
            for (i, d) in divs.iter().enumerate() {
                if *d <= r {
                    cur.push(*d);
                    pick(&divs[i + 1..], r - d, k - 1, cur, out);
                    cur.pop();
                }
            }
        }
        pick(&divs, r, 3, &mut Vec::new(), &mut subsets);
        for s in subsets {
            let mut parts = vec![m];
            parts.extend(s.iter().map(|d| m * n / d));
            parts.sort();
            parts.dedup();
            if parts.len() == s.len() + 1 && *parts.last().unwrap() <= 1000 {
                out.push(parts);
            }
        }
    }
    out
}

fn divisor_fewest(n: u64) -> Option<Vec<u64>> {
    divisor_sums(n)
        .into_iter()
        .min_by_key(|c| (c.len(), *c.last().unwrap()))
}

fn divisor_small_last(n: u64) -> Option<Vec<u64>> {
    divisor_sums(n)
        .into_iter()
        .min_by_key(|c| (*c.last().unwrap(), c.len()))
}

/// The divisor method with the fewest terms, and of those the one whose
/// first part is smallest, unless that makes the last part larger than 1000.
fn divisor_first(n: u64) -> Option<Vec<u64>> {
    divisor_sums(n)
        .into_iter()
        .min_by_key(|c| (c.len(), c[0], *c.last().unwrap()))
}

pub const RULES: &[Rule] = &[
    Rule {
        name: "greedy",
        what: "the largest part that fits, again and again",
        choose: greedy,
    },
    Rule {
        name: "shortest",
        what: "fewest parts (at most 4, none past 1000), then the smallest last part",
        choose: shortest,
    },
    Rule {
        name: "shortest-even",
        what: "fewest parts, then fewest odd parts, then the smallest last part",
        choose: shortest_even,
    },
    Rule {
        name: "smallest-last",
        what: "the smallest last part, then fewest parts",
        choose: smallest_last,
    },
    Rule {
        name: "divisors",
        what: "2/n = m̅ + (2m-n)/mn, 2m-n made of divisors of m; fewest parts, then the smallest last part",
        choose: divisor_fewest,
    },
    Rule {
        name: "divisors-last",
        what: "the divisor method, the smallest last part first",
        choose: divisor_small_last,
    },
    Rule {
        name: "divisors-first",
        what: "the divisor method, fewest parts, then the smallest first part",
        choose: divisor_first,
    },
];

pub struct Score {
    pub rule: &'static Rule,
    pub matched: Vec<u64>,
    pub missed: Vec<(u64, Vec<u64>)>,
}

pub fn score() -> Vec<Score> {
    RULES
        .iter()
        .map(|rule| {
            let mut matched = Vec::new();
            let mut missed = Vec::new();
            for (n, parts) in table::TABLE {
                let got = (rule.choose)(*n).unwrap_or_default();
                if got == parts.to_vec() {
                    matched.push(*n);
                } else {
                    missed.push((*n, got));
                }
            }
            Score { rule, matched, missed }
        })
        .collect()
}

/// Entries no rule here reproduces.
pub fn unexplained() -> Vec<u64> {
    let scores = score();
    table::TABLE
        .iter()
        .map(|e| e.0)
        .filter(|n| scores.iter().all(|s| !s.matched.contains(n)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rules_give_two_parts() {
        for r in RULES {
            for n in [5u64, 7, 13, 29, 101] {
                if let Some(p) = (r.choose)(n) {
                    let s: Rat = p.iter().map(|d| Rat::part(*d)).sum();
                    assert_eq!(s, Rat::new(2, n as i128), "{} for {}", r.name, n);
                }
            }
        }
    }
}
