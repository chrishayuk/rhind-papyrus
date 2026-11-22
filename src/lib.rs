//! The Rhind Mathematical Papyrus, run.
//!
//! `page` reads the notation, `number` and `rat` hold the scribe's numbers,
//! `table` is the papyrus's table of 2 ÷ n, `scribe` does the scribe's
//! operations, and `check` runs a page and compares.

pub mod check;
pub mod json;
pub mod lsp;
pub mod number;
pub mod page;
pub mod papyrus;
pub mod rat;
pub mod routes;
pub mod rules;
pub mod scribe;
pub mod slips;
pub mod table;
