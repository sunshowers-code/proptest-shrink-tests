mod args;
mod bad_type;
mod test_functions;
mod test_utils;

use crate::{args::Args, test_utils::run_all};
use bad_type::{
    generate_bad_type_pair_flat_map, generate_bad_type_pair_map, generate_bad_type_triple_flat_map,
    generate_bad_type_triple_map,
};
use clap::Parser;
use proptest::collection::vec;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use test_functions::{test_bad_type_pair, test_bad_type_triple};
use test_utils::write_tsv;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let mut all_results = Vec::new();

    let strategy = vec(generate_bad_type_pair_flat_map(), 0..128);
    all_results.push(run_all(
        "bad_type_pair_flat_map",
        args.iter,
        &strategy,
        test_bad_type_pair,
    ));

    let strategy = vec(generate_bad_type_pair_map(), 0..128);
    all_results.push(run_all(
        "bad_type_pair_map",
        args.iter,
        &strategy,
        test_bad_type_pair,
    ));

    let strategy = vec(generate_bad_type_triple_flat_map(), 0..128);
    all_results.push(run_all(
        "bad_type_triple_flat_map",
        args.iter,
        &strategy,
        test_bad_type_triple,
    ));

    let strategy = vec(generate_bad_type_triple_map(), 0..128);
    all_results.push(run_all(
        "bad_type_triple_map",
        args.iter,
        &strategy,
        test_bad_type_triple,
    ));

    let output_file = args.output_file;

    // Turn the results into tab-separated values.
    let mut f = BufWriter::new(File::create(&output_file)?);
    write_tsv(&all_results, &mut f)?;
    f.flush()?;

    eprintln!("results written to {}", output_file);

    Ok(())
}
