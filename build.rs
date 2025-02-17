use camino::Utf8PathBuf;
use std::{env, fs::File, io::Write};

fn main() {
    // Read the opt-level and use it to generate the output file name
    println!("cargo::rerun-if-env-changed=OPT_LEVEL");

    let opt_level = env::var("OPT_LEVEL").unwrap_or_else(|_| "unknown".into());
    let output_file = format!("results-opt-level-{}.tsv", opt_level);

    let out_dir =
        Utf8PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR environment variable is set"));
    let mut out_file = File::create(out_dir.join("extra.rs")).unwrap();

    writeln!(
        out_file,
        "static DEFAULT_OUTPUT_FILE: &str = \"{}\";",
        output_file
    )
    .unwrap();
}
