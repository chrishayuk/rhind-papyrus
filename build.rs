// Compile the transcription of the papyrus into the program, so that
// `rhind verify` and the playground carry it with them.
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=papyrus");
    let mut names: Vec<String> = fs::read_dir("papyrus")
        .map(|d| {
            d.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.ends_with(".rmp"))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let dir = env::current_dir().unwrap();
    let mut out = String::from("pub const PAPYRUS: &[(&str, &str)] = &[\n");
    for n in &names {
        let p = dir.join("papyrus").join(n);
        println!("cargo:rerun-if-changed={}", p.display());
        out.push_str(&format!(
            "    ({:?}, include_str!({:?})),\n",
            n.trim_end_matches(".rmp"),
            p.display().to_string()
        ));
    }
    out.push_str("];\n");
    fs::write(Path::new(&env::var("OUT_DIR").unwrap()).join("papyrus.rs"), out).unwrap();
}
