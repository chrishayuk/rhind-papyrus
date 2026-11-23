use rhind::check::{self, Severity};
use rhind::number::{self, Eg};
use rhind::{papyrus, scribe, slips, table};
use std::io::Read;
use std::process::exit;

const USAGE: &str = "rhind: the Rhind Mathematical Papyrus, run

  rhind FILE                 the page, with every figure the scribe wrote checked
  rhind no 27                a problem of the papyrus, run (also: rhind no 7B, rhind table)
  rhind --plain FILE         the page in the scribe's figures, without marks
  rhind --check FILE         what is wrong with it, a line each
  rhind verify               run the whole papyrus and compare with the editions
  rhind errata               every wrong figure in the papyrus, and the slips that would make it
  rhind multiply 3 '2 by 5   the working, as the scribe would write it
  rhind reckon with 6 to find 21
  rhind double '7            (also halve, thirds: two-thirds of)
  rhind routes               does the scribe work each column the way rhind does?
  rhind routes --derive      is each row of a column made from one before it, by operations the papyrus shows?
  rhind 2/n                  how were the entries of the 2 ÷ n table chosen? rules tested against the papyrus
  rhind decisions            the choices this reconstruction makes that the papyrus does not
  rhind lsp                  a language server on standard input and output
  rhind json FILE            the run, as JSON, for the playground

  FILE may be - for standard input. In a figure, 'n is one part in n and 2/3 is two-thirds.
";

fn read(path: &str) -> String {
    if path == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        return s;
    }
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("rhind: {}: {}", path, e);
        exit(2)
    })
}

fn eg(s: &str) -> Eg {
    match number::parse(s) {
        Ok(q) if q.segments.len() == 1 => q.eg().clone(),
        Ok(_) => {
            eprintln!("rhind: `{}`: one number, without units", s);
            exit(2)
        }
        Err(e) => {
            eprintln!("rhind: {}", e);
            exit(2)
        }
    }
}

fn split(args: &[String], word: &str) -> (String, String) {
    let joined = args.join(" ");
    match joined.split_once(&format!(" {} ", word)) {
        Some((a, b)) => (a.to_string(), b.to_string()),
        None => {
            eprintln!("rhind: expected `… {} …`", word);
            exit(2)
        }
    }
}

fn print_rows(rows: &[scribe::Row]) {
    let w = rows
        .iter()
        .map(|r| r.left.to_string().chars().count())
        .max()
        .unwrap_or(1);
    for r in rows {
        let l = r.left.to_string();
        println!(
            "{} {}{}   {}",
            if r.tick { "\\" } else { " " },
            " ".repeat(w - l.chars().count()),
            l,
            r.right
        );
    }
}

fn page(text: &str, marks: bool) -> i32 {
    let run = check::run(text);
    print!("{}", check::render(&run, marks));
    if !marks {
        return 0;
    }
    let wrong: Vec<_> = run.findings.iter().filter(|f| f.severity == Severity::Wrong).collect();
    let notation: Vec<_> = run
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Notation)
        .collect();
    let (checked, right) = run
        .sections
        .iter()
        .fold((0, 0), |a, s| (a.0 + s.checked, a.1 + s.right));
    let (checked, right) = if run.sections.is_empty() {
        let c = run
            .marks
            .iter()
            .filter(|m| matches!(m, check::Mark::Right | check::Mark::Wrong(_)))
            .count();
        let r = run.marks.iter().filter(|m| matches!(m, check::Mark::Right)).count();
        (c, r)
    } else {
        (checked, right)
    };
    println!();
    println!(
        "{} figures checked, {} right, {} wrong.",
        checked,
        right,
        checked - right
    );
    for f in &wrong {
        println!("line {}: {}", f.line + 1, f.message);
        if let Some(c) = f.carried {
            println!(
                "    {}",
                if c {
                    "the error is carried into what follows"
                } else {
                    "what follows is right: the error is not carried"
                }
            );
        }
        for s in slips::explain(&run, f) {
            println!("    could be {}: {}", s.model, s.explanation);
        }
    }
    for f in &notation {
        println!("line {}: not in the notation: {}", f.line + 1, f.message);
    }
    if notation.is_empty() {
        0
    } else {
        1
    }
}

fn check_only(path: &str, text: &str) -> i32 {
    let run = check::run(text);
    for f in &run.findings {
        let sev = match f.severity {
            Severity::Wrong => "wrong",
            Severity::Form => "form",
            Severity::Unchecked => "unchecked",
            Severity::Notation => "error",
        };
        println!("{}:{}:{}: {}: {}", path, f.line + 1, f.span.0 + 1, sev, f.message);
    }
    if run.findings.iter().any(|f| f.severity == Severity::Notation) {
        1
    } else {
        0
    }
}

fn verify() -> i32 {
    let report = papyrus::verify();
    let mut total = (0, 0, 0, 0);
    let mut problems = 0;
    println!(
        "{:<14} {:>8} {:>7} {:>7} {:>12}",
        "", "checked", "right", "wrong", "not checked"
    );
    for (name, run) in &report.runs {
        let (c, r, w, u) = run.sections.iter().fold((0, 0, 0, 0), |a, s| {
            (a.0 + s.checked, a.1 + s.right, a.2 + s.wrong, a.3 + s.unchecked)
        });
        problems += run.sections.iter().filter(|s| s.heading.starts_with("No.")).count();
        total = (total.0 + c, total.1 + r, total.2 + w, total.3 + u);
        let label = name.trim_start_matches("no-").trim_start_matches('0');
        println!("{:<14} {:>8} {:>7} {:>7} {:>12}", label, c, r, w, u);
    }
    println!(
        "{:<14} {:>8} {:>7} {:>7} {:>12}",
        "all", total.0, total.1, total.2, total.3
    );
    println!();
    let noted = report.wrong.iter().filter(|w| w.noted).count();
    let of = |k: papyrus::Kind| report.wrong.iter().filter(|w| w.kind == k).count();
    println!(
        "{} problems. {} lines checked: {} agree with the working, {} do not.",
        problems, total.0, total.1, total.2
    );
    println!(
        "{} diagnostic records on those lines (a written figure can give more than one):",
        report.wrong.len()
    );
    println!(
        "  {:>3}  differ only in a part sign: a notation the editions treat differently, not necessarily an error",
        of(papyrus::Kind::Notation)
    );
    println!(
        "  {:>3}  the ticked rows do not make the total",
        of(papyrus::Kind::Selection)
    );
    println!(
        "  {:>3}  a figure differs in more than its part sign",
        of(papyrus::Kind::Figure)
    );
    println!(
        "The transcribers' notes mention an edition's comment at {} of the {} (a keyword search of the notes, not a census of what the editions say).",
        noted,
        report.wrong.len()
    );
    if !report.unmatched.is_empty() {
        let checked = report.unmatched.iter().filter(|u| u.4).count();
        println!(
            "The notes mention an edition's comment at {} more places where rhind reports nothing: {} on lines it checks and finds right, {} on words, signs, units or figures it does not check (see `rhind errata`).",
            report.unmatched.len(),
            checked,
            report.unmatched.len() - checked
        );
    }
    let diffs = papyrus::table_differences();
    if !diffs.is_empty() {
        println!(
            "The 2 ÷ n table as transcribed differs from the table rhind doubles with at {} entries:",
            diffs.len()
        );
        for (n, want, got) in diffs {
            println!("  2 ÷ {}: {:?} in rhind, {:?} as transcribed", n, want, got);
        }
    }
    let bad: usize = report
        .runs
        .iter()
        .map(|(_, r)| r.findings.iter().filter(|f| f.severity == Severity::Notation).count())
        .sum();
    if bad > 0 {
        println!(
            "{} lines of the transcription are not in the notation (rhind --check papyrus/FILE).",
            bad
        );
        return 1;
    }
    0
}

fn errata() {
    let report = papyrus::verify();
    for w in &report.wrong {
        let f = &w.finding;
        println!(
            "{} line {} ({}) [{}]: {}",
            w.file,
            f.line + 1,
            w.heading,
            match w.kind {
                papyrus::Kind::Notation => "notation",
                papyrus::Kind::Selection => "selection",
                papyrus::Kind::Figure => "figure",
            },
            f.message
        );
        if let Some(c) = f.carried {
            println!(
                "    {}",
                if c {
                    "carried into what follows"
                } else {
                    "not carried: what follows is right"
                }
            );
        }
        for s in &w.slips {
            println!("    could be {}: {}", s.model, s.explanation);
        }
        match &w.note {
            Some(n) if w.noted => println!("    noted: {}", n),
            _ => println!("    not pointed out by either edition, as transcribed"),
        }
    }
    if !report.unmatched.is_empty() {
        println!();
        println!("Pointed out by an edition, on a line rhind checks and finds right:");
        for (file, line, heading, note, _) in report.unmatched.iter().filter(|u| u.4) {
            println!("{} line {} ({}): {}", file, line + 1, heading, note);
        }
        println!();
        println!("Pointed out by an edition, on a line rhind does not check:");
        for (file, line, heading, note, _) in report.unmatched.iter().filter(|u| !u.4) {
            println!("{} line {} ({}): {}", file, line + 1, heading, note);
        }
    }
    // which signs are written for which: the slips as a table
    let mut pairs: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    // for each model: the figures it fits, and those it is the only fit for
    let mut fits: std::collections::BTreeMap<&str, (usize, usize)> = Default::default();
    let mut none = 0;
    for w in &report.wrong {
        let mut seen: Vec<&str> = w.slips.iter().map(|s| s.model).collect();
        seen.dedup();
        for m in &seen {
            let e = fits.entry(m).or_default();
            e.0 += 1;
            if seen.len() == 1 {
                e.1 += 1;
            }
        }
        for s in &w.slips {
            if s.model == "one-sign" || s.model == "multiplier" || s.model == "in-the-list" {
                if let Some(p) = s.explanation.split(" written for ").collect::<Vec<_>>().get(0..2) {
                    let a: String = p[0].rsplit(' ').next().unwrap_or("").to_string();
                    let b: String = p[1].split([' ', ':', ',', '(']).next().unwrap_or("").to_string();
                    if !a.is_empty() && a.chars().all(|c| c.is_ascii_digit()) && b.chars().all(|c| c.is_ascii_digit()) {
                        pairs
                            .entry(format!("{} written for {}", a, b))
                            .or_default()
                            .push(w.heading.to_string());
                    }
                }
            }
        }
        if w.slips.is_empty() {
            none += 1;
        }
    }
    println!();
    println!(
        "The slips that fit the {} wrong figures: a figure can fit more than one, so the",
        report.wrong.len()
    );
    println!("first column adds to more than the total. These are explanations that fit, not causes.");
    println!("  {:<12} {:>5} {:>10}", "", "fits", "only fit");
    let mut rows: Vec<_> = fits.into_iter().collect();
    rows.sort_by_key(|r| std::cmp::Reverse(r.1 .0));
    for (m, (n, only)) in rows {
        println!("  {:<12} {:>5} {:>10}", m, n, only);
    }
    println!("  {:<12} {:>5}", "none fits", none);
    println!();
    println!("One sign written for another:");
    let mut pairs: Vec<_> = pairs.into_iter().collect();
    pairs.sort_by_key(|p| std::cmp::Reverse(p.1.len()));
    for (pair, wheres) in pairs {
        println!("  {:<22} {}x  {}", pair, wheres.len(), wheres.join(", "));
    }
    println!();
    println!("The models:");
    for (m, d) in slips::MODELS {
        println!("  {:<11} {}", m, d);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        print!("{}", USAGE);
        exit(2)
    };
    let rest = &args[1..];
    let code = match cmd.as_str() {
        "-h" | "--help" | "help" => {
            print!("{}", USAGE);
            0
        }
        "verify" | "--verify" => verify(),
        "errata" => {
            errata();
            0
        }
        "decisions" => {
            for (k, d) in scribe::DECISIONS {
                println!("{}\n    {}\n", k, d);
            }
            0
        }
        "no" | "table" => {
            let name = if cmd == "table" {
                "table-2n-a".to_string()
            } else {
                format!("no-{}", rest.join(""))
            };
            let text: String = if cmd == "table" {
                papyrus::files()
                    .iter()
                    .filter(|(n, _)| n.starts_with("table-"))
                    .map(|(_, t)| *t)
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                match papyrus::get(&name) {
                    Some(t) => t.to_string(),
                    None => {
                        eprintln!("rhind: no problem {} in the papyrus as transcribed", rest.join(""));
                        exit(2)
                    }
                }
            };
            page(&text, true)
        }
        "multiply" => {
            let (a, b) = split(rest, "by");
            let rows = scribe::multiply(&eg(&a), &eg(&b));
            println!("Multiply {} by {}.", eg(&a), eg(&b));
            print_rows(&rows);
            println!("Total {}.", scribe::answer(&rows, false));
            0
        }
        "reckon" => {
            let r: Vec<String> = rest.iter().skip_while(|w| *w == "with").cloned().collect();
            let (a, b) = split(&r, "to find");
            let rows = scribe::reckon(&eg(&a), &eg(&b));
            println!("Reckon with {} to find {}.", eg(&a), eg(&b));
            print_rows(&rows);
            println!("Result {}.", scribe::answer(&rows, true));
            0
        }
        "double" | "halve" | "thirds" => {
            let x = eg(&rest.join(" "));
            let (got, want) = match cmd.as_str() {
                "double" => (scribe::double(&x), x.value() * rhind::rat::Rat::int(2)),
                "halve" => (scribe::halve(&x), x.value() / rhind::rat::Rat::int(2)),
                _ => (scribe::two_thirds(&x), x.value() * rhind::rat::Rat::new(2, 3)),
            };
            if got.value() != want {
                eprintln!(
                    "rhind: {} cannot be written exactly in parts here (it is {})",
                    want, got
                );
                exit(1)
            }
            println!("{}", got);
            if cmd == "double" {
                let outside: Vec<u64> = x
                    .parts()
                    .into_iter()
                    .filter(|n| n % 2 == 1 && *n > 3 && table::twice(*n).is_none())
                    .collect();
                if !outside.is_empty() {
                    eprintln!(
                        "note: twice {} is not in the papyrus's table (it stops at 101); rhind wrote it by value",
                        outside
                            .iter()
                            .map(|n| number::overbar(*n))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
            }
            0
        }
        "lsp" => match rhind::lsp::serve(&mut std::io::stdin().lock(), &mut std::io::stdout()) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("rhind lsp: {}", e);
                1
            }
        },
        "json" => {
            let text = read(rest.first().map(|s| s.as_str()).unwrap_or("-"));
            println!("{}", rhind::lsp::run_json(&text));
            0
        }
        "--plain" => page(&read(rest.first().map(|s| s.as_str()).unwrap_or("-")), false),
        "--check" => {
            let p = rest.first().map(|s| s.as_str()).unwrap_or("-");
            check_only(p, &read(p))
        }
        "routes" => {
            use rhind::routes::{Fidelity, Verdict};
            let show = |v: &[rhind::rat::Rat]| {
                v.iter()
                    .map(|x| scribe::express(*x).to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let readings = |rs: &[rhind::routes::Reading]| {
                for r in rs {
                    println!(
                        "    line {}: {} read as {} ({})",
                        r.line + 1,
                        scribe::express(r.written),
                        scribe::express(r.read),
                        r.why
                    );
                }
            };
            if rest.first().map(|s| s == "--derive").unwrap_or(false) {
                let all = rhind::routes::derive();
                let listed = rest.iter().any(|s| s == "--all");
                println!("Is every row of the scribe's column made from a row already written, by one of these, and every row used?\n");
                for (_, what) in rhind::routes::OPS {
                    println!("  {}", what);
                }
                println!("\nThe list is fixed; a column that needs another operation is reported, not accommodated.");
                println!(
                    "A row is used if it is ticked or another row is made from it. Products are checked elsewhere.\n"
                );
                type Set<'a> = &'a dyn Fn(&rhind::routes::Derivation) -> bool;
                let count = |set: Set, f: Fidelity| all.iter().filter(|d| set(d) && d.fidelity == f).count();
                let sets: [(&str, Set); 4] = [
                    ("the columns routes compares", &|d| d.compared),
                    ("  of which rhind's route matches", &|d| {
                        d.verdict.map(|v| v.matches()).unwrap_or(false)
                    }),
                    ("  of which it does not", &|d| {
                        d.compared && !d.verdict.map(|v| v.matches()).unwrap_or(false)
                    }),
                    ("every column read in full, beginning with 1", &|_| true),
                ];
                println!(
                    "  {:<46} {:>8} {:>8} {:>8} {:>10} {:>6}",
                    "", "faithful", "no ticks", "unused", "underived", "all"
                );
                for (name, set) in sets {
                    println!(
                        "  {:<46} {:>8} {:>8} {:>8} {:>10} {:>6}",
                        name,
                        count(set, Fidelity::Faithful),
                        count(set, Fidelity::NoTicks),
                        count(set, Fidelity::Unused),
                        count(set, Fidelity::Underived),
                        all.iter().filter(|d| set(d)).count()
                    );
                }
                let theirs = |f: Fidelity| all.iter().filter(|d| d.rhind == Some(f)).count();
                println!(
                    "  {:<46} {:>8} {:>8} {:>8} {:>10} {:>6}",
                    "rhind's own routes for the columns compared",
                    theirs(Fidelity::Faithful),
                    theirs(Fidelity::NoTicks),
                    theirs(Fidelity::Unused),
                    theirs(Fidelity::Underived),
                    all.iter().filter(|d| d.rhind.is_some()).count()
                );
                let split = |m: bool, f: Fidelity| {
                    all.iter()
                        .filter(|d| {
                            d.compared && d.verdict.map(|v| v.matches()).unwrap_or(false) == m && d.rhind == Some(f)
                        })
                        .count()
                };
                println!(
                    "  {:<46} {:>8} {:>8} {:>8} {:>10} {:>6}",
                    "  where it differs from the scribe's",
                    split(false, Fidelity::Faithful),
                    split(false, Fidelity::NoTicks),
                    split(false, Fidelity::Unused),
                    split(false, Fidelity::Underived),
                    all.iter()
                        .filter(|d| d.compared && !d.verdict.map(|v| v.matches()).unwrap_or(false))
                        .count()
                );
                println!();
                let mut ops: std::collections::BTreeMap<rhind::routes::Op, usize> = Default::default();
                for d in &all {
                    for r in &d.rows {
                        if let Some((op, _)) = r.2 {
                            *ops.entry(op).or_default() += 1;
                        }
                    }
                }
                print!("rows made by:");
                for (op, n) in &ops {
                    print!("  {} {}", op.name(), n);
                }
                println!("\n");
                if !listed {
                    println!("rhind routes --derive --all lists the columns that are not faithful, row by row.");
                    exit(0);
                }
                for d in all
                    .iter()
                    .filter(|d| d.fidelity != Fidelity::Faithful || !d.readings.is_empty())
                {
                    println!(
                        "{} line {}: {:?}{}",
                        d.file,
                        d.line + 1,
                        d.fidelity,
                        d.verdict.map(|v| format!(" (routes: {:?})", v)).unwrap_or_default()
                    );
                    let mut cells = Vec::new();
                    for (i, (m, tick, how)) in d.rows.iter().enumerate() {
                        let mark = if *tick { "\\" } else { "" };
                        let how = match how {
                            None if i == 0 => String::new(),
                            None => " [none]".into(),
                            Some((op, j)) if *j + 1 == i => format!(" [{}]", op.name()),
                            Some((op, j)) => format!(" [{} of {}]", op.name(), scribe::express(d.rows[*j].0)),
                        };
                        let unused = if d.unused.contains(&i) { " [unused]" } else { "" };
                        cells.push(format!("{}{}{}{}", mark, scribe::express(*m), how, unused));
                    }
                    println!("    {}", cells.join("  "));
                    readings(&d.readings);
                }
                exit(0);
            }
            let all = rhind::routes::routes();
            let count = |v: Verdict| all.iter().filter(|r| r.verdict == v).count();
            let (all_cols, clean, eligible) = rhind::routes::census();
            println!("The problems have {} columns of two rows or more; {} can be read in full (nothing lost, restored or doubtful);", all_cols, clean);
            println!("{} of those begin with a row for 1, say what they multiply, and tick at least one row. Only those {} are compared.", eligible, eligible);
            println!("A column that matches has the same multipliers and ticks as rhind's route; the products are checked elsewhere.");
            println!("A multiplier written without its part sign, where the check finds that, is read as the part.\n");
            for (v, what) in [
                (Verdict::Same, "the same rows and ticks, by doubling"),
                (Verdict::SameTens, "the same, multiplying the tens by a row for 10"),
                (Verdict::SameThird, "the same, taking 3̅ straight from the first row"),
                (Verdict::SameReckon, "the same, read as a division"),
                (Verdict::Ticks, "the same rows, ticked differently"),
                (Verdict::Extra, "rhind's rows, with others written besides"),
                (Verdict::Tens, "the scribe uses 10 another way"),
                (
                    Verdict::Thirds,
                    "the scribe goes by ⅔ where rhind halves, or the reverse",
                ),
                (Verdict::Other, "another route"),
            ] {
                println!("  {:>3}  {}", count(v), what);
            }
            println!("  {:>3}  columns in all\n", all.len());
            if rest.first().map(|s| s == "--all").unwrap_or(false) {
                for r in all
                    .iter()
                    .filter(|r| r.verdict != Verdict::Same || !r.readings.is_empty())
                {
                    println!(
                        "{} line {}: {:?}\n    scribe {}\n    rhind  {}",
                        r.file,
                        r.line + 1,
                        r.verdict,
                        show(&r.scribe),
                        show(&r.rhind)
                    );
                    readings(&r.readings);
                }
            } else {
                println!("rhind routes --all lists the columns that differ; rhind routes --derive asks whether each row is made from one before it.");
            }
            0
        }
        "2/n" => {
            let show = |p: &[u64]| {
                Eg {
                    whole: 0,
                    terms: p.iter().map(|d| number::Term::Part(*d)).collect(),
                }
                .to_string()
            };
            if rest.first().map(|s| s == "--table").unwrap_or(false) {
                for (n, parts) in table::TABLE {
                    println!("2 ÷ {:<3} = {}", n, show(parts));
                }
                exit(0);
            }
            println!("How were the entries of the 2 ÷ n table chosen? Each rule, run for every odd n from 5 to 101:\n");
            let scores = rhind::rules::score();
            for s in &scores {
                println!(
                    "{:<15} {:>2} of {}   {}",
                    s.rule.name,
                    s.matched.len(),
                    table::TABLE.len(),
                    s.rule.what
                );
            }
            let best = scores.iter().max_by_key(|s| s.matched.len()).unwrap();
            println!("\nWhere `{}` and the papyrus differ:", best.rule.name);
            for (n, got) in &best.missed {
                println!(
                    "  2 ÷ {:<3} papyrus {:<28} rule {}",
                    n,
                    show(table::twice(*n).unwrap()),
                    if got.is_empty() {
                        "(no candidate within the bounds)".to_string()
                    } else {
                        show(got)
                    }
                );
            }
            println!("\nThe bounds: at most 4 parts, none past 1000; the divisor rules take m with n/2 < m ≤ n and at most 3 divisors.");
            println!("The scores depend on them: raising the cap to 10000 changes `divisors` from 32 to 25 (`divisors-last` stays 34).");
            println!("The targets are the table as rhind uses it, with 2 ÷ 43, 59 and 93 as the working, not the answers, give them.");
            println!("A rule's agreement with the table describes it; it does not show the scribe used that rule.");
            let none = rhind::rules::unexplained();
            println!(
                "\nNo rule here gives: {}",
                none.iter().map(|n| format!("2 ÷ {}", n)).collect::<Vec<_>>().join(", ")
            );
            0
        }
        path => page(&read(path), true),
    };
    exit(code)
}
