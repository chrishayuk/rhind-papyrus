//! The table at the head of the papyrus: two divided by each odd number from
//! 3 to 101, as a sum of distinct unit fractions.
//!
//! This is the table the scribe used to double an odd part: twice 7̅ is
//! 4̅ 2̅8̅ because the table says so. The engine uses it the same way, and has
//! no other way to double an odd part.
//!
//! The entries are the papyrus's own, as both editions read them. They are
//! checked against the transcription of the table (`papyrus/table-2n-*.rmp`)
//! by `rhind verify` and by `tests/papyrus.rs`.

pub const TABLE: &[(u64, &[u64])] = &[
    (5, &[3, 15]),
    (7, &[4, 28]),
    (9, &[6, 18]),
    (11, &[6, 66]),
    (13, &[8, 52, 104]),
    (15, &[10, 30]),
    (17, &[12, 51, 68]),
    (19, &[12, 76, 114]),
    (21, &[14, 42]),
    (23, &[12, 276]),
    (25, &[15, 75]),
    (27, &[18, 54]),
    (29, &[24, 58, 174, 232]),
    (31, &[20, 124, 155]),
    (33, &[22, 66]),
    (35, &[30, 42]),
    (37, &[24, 111, 296]),
    (39, &[26, 78]),
    (41, &[24, 246, 328]),
    (43, &[42, 86, 129, 301]),
    (45, &[30, 90]),
    (47, &[30, 141, 470]),
    (49, &[28, 196]),
    (51, &[34, 102]),
    (53, &[30, 318, 795]),
    (55, &[30, 330]),
    (57, &[38, 114]),
    (59, &[36, 236, 531]),
    (61, &[40, 244, 488, 610]),
    (63, &[42, 126]),
    (65, &[39, 195]),
    (67, &[40, 335, 536]),
    (69, &[46, 138]),
    (71, &[40, 568, 710]),
    (73, &[60, 219, 292, 365]),
    (75, &[50, 150]),
    (77, &[44, 308]),
    (79, &[60, 237, 316, 790]),
    (81, &[54, 162]),
    (83, &[60, 332, 415, 498]),
    (85, &[51, 255]),
    (87, &[58, 174]),
    (89, &[60, 356, 534, 890]),
    (91, &[70, 130]),
    (93, &[62, 186]),
    (95, &[60, 380, 570]),
    (97, &[56, 679, 776]),
    (99, &[66, 198]),
    (101, &[101, 202, 303, 606]),
];

/// Twice n̅, from the table. `None` outside it (n even, or above 101).
pub fn twice(n: u64) -> Option<&'static [u64]> {
    TABLE.iter().find(|e| e.0 == n).map(|e| e.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rat::Rat;
    #[test]
    fn every_entry_is_two_parts() {
        for (n, parts) in TABLE {
            let sum: Rat = parts.iter().map(|d| Rat::part(*d)).sum();
            assert_eq!(sum, Rat::new(2, *n as i128), "2 ÷ {}", n);
            assert!(parts.windows(2).all(|w| w[0] < w[1]), "2 ÷ {} in order", n);
        }
        assert_eq!(TABLE.len(), 49);
    }
}
