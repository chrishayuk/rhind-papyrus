//! The papyrus, run: every page is in the notation, and what running it finds
//! is what it found last time. A change in what rhind finds shows as a
//! difference here; if it is meant, `RHIND_BLESS=1 cargo test` writes the new
//! findings, and the difference is reviewed in the commit.

use std::path::Path;
use std::process::Command;

fn rhind(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_rhind"))
        .args(args)
        .output()
        .expect("rhind runs");
    String::from_utf8(out.stdout).unwrap()
}

fn same_as(name: &str, got: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/expected").join(name);
    if std::env::var("RHIND_BLESS").is_ok() {
        std::fs::write(&path, got).unwrap();
        return;
    }
    let want = std::fs::read_to_string(&path).unwrap_or_default();
    if want != got {
        let first = want
            .lines()
            .zip(got.lines())
            .position(|(a, b)| a != b)
            .unwrap_or(want.lines().count().min(got.lines().count()));
        panic!(
            "{} differs from tests/expected/{} at line {}:\n  was: {}\n  now: {}\n(RHIND_BLESS=1 cargo test to accept)",
            name,
            name,
            first + 1,
            want.lines().nth(first).unwrap_or(""),
            got.lines().nth(first).unwrap_or("")
        );
    }
}

#[test]
fn every_page_is_in_the_notation() {
    for entry in std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("papyrus")).unwrap() {
        let path = entry.unwrap().path();
        let out = rhind(&["--check", path.to_str().unwrap()]);
        let bad: Vec<&str> = out.lines().filter(|l| l.contains(": error: ")).collect();
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
}

#[test]
fn the_papyrus_verifies_as_before() {
    same_as("verify.txt", &rhind(&["verify"]));
}

#[test]
fn the_errata_are_as_before() {
    same_as("errata.txt", &rhind(&["errata"]));
}

#[test]
fn the_table_rules_score_as_before() {
    same_as("2n.txt", &rhind(&["2/n"]));
}

#[test]
fn every_example_prints_its_page() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut n = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "rmp").unwrap_or(false) {
            let want = std::fs::read_to_string(path.with_extension("out")).unwrap();
            assert_eq!(rhind(&[path.to_str().unwrap()]), want, "{}", path.display());
            n += 1;
        }
    }
    assert!(n >= 5);
}

#[test]
fn the_commands() {
    assert!(rhind(&["multiply", "3", "'2", "by", "5"]).contains("Total 17 2̅."));
    assert!(rhind(&["reckon", "with", "7", "to", "find", "2"]).contains("Result 4̅ 2̅8̅."));
    assert_eq!(rhind(&["double", "'7"]).trim(), "4̅ 2̅8̅");
    assert!(rhind(&["decisions"]).contains("outvote"));
}
