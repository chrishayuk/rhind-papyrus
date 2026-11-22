//! Every page and every command in the tutorial is run, and what it prints is
//! compared with what the tutorial says it prints. `RHIND_BLESS=1 cargo test`
//! writes what it prints into the tutorial instead.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn rhind(args: &[String], input: Option<&str>) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rhind"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("rhind runs");
    if let Some(text) = input {
        child.stdin.take().unwrap().write_all(text.as_bytes()).unwrap();
    } else {
        drop(child.stdin.take());
    }
    let out = child.wait_with_output().unwrap();
    String::from_utf8(out.stdout).unwrap().trim_end().to_string()
}

/// Split a command line the way the shell would, for the simple quoting the
/// tutorial uses (none: `'7` is passed as it is).
fn words(line: &str) -> Vec<String> {
    line.split_whitespace().skip(1).map(String::from).collect()
}

/// The chapter with every output worked out afresh.
fn worked(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line == "```rmp" {
            // the page, then its output in the next fence
            let mut page = String::new();
            out.push(line.to_string());
            i += 1;
            while lines[i] != "```" {
                page.push_str(lines[i]);
                page.push('\n');
                out.push(lines[i].to_string());
                i += 1;
            }
            out.push("```".into());
            i += 1;
            while lines[i].trim().is_empty() {
                out.push(lines[i].to_string());
                i += 1;
            }
            assert!(lines[i].starts_with("```"), "a page must be followed by its output");
            out.push(lines[i].to_string());
            i += 1;
            while lines[i] != "```" {
                i += 1;
            }
            out.push(rhind(&["-".to_string()], Some(&page)));
            out.push("```".into());
            i += 1;
            continue;
        }
        if line == "```sh" {
            out.push(line.to_string());
            i += 1;
            while lines[i] != "```" {
                if let Some(cmd) = lines[i].strip_prefix("$ ") {
                    out.push(lines[i].to_string());
                    out.push(rhind(&words(cmd), None));
                    i += 1;
                    while lines[i] != "```" && !lines[i].starts_with("$ ") {
                        i += 1;
                    }
                } else {
                    out.push(lines[i].to_string());
                    i += 1;
                }
            }
            out.push("```".into());
            i += 1;
            continue;
        }
        out.push(line.to_string());
        i += 1;
    }
    let mut s = out.join("\n");
    s.push('\n');
    s
}

#[test]
fn every_sitting_in_the_tutorial() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tutorial");
    let mut chapters: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
        .collect();
    chapters.sort();
    assert!(chapters.len() >= 9);
    for path in chapters {
        let text = std::fs::read_to_string(&path).unwrap();
        let fresh = worked(&text);
        if std::env::var("RHIND_BLESS").is_ok() {
            std::fs::write(&path, &fresh).unwrap();
            continue;
        }
        if fresh != text {
            let at = text.lines().zip(fresh.lines()).position(|(a, b)| a != b).unwrap_or(0);
            panic!(
                "{} line {}:\n  says: {}\n  gets: {}",
                path.display(),
                at + 1,
                text.lines().nth(at).unwrap_or(""),
                fresh.lines().nth(at).unwrap_or("")
            );
        }
    }
}
