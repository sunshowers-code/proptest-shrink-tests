use camino::Utf8PathBuf;
use clap::Parser;

include!(concat!(env!("OUT_DIR"), "/extra.rs"));

#[derive(Debug, Parser)]
pub struct Args {
    /// Number of iterations to run the test with.
    #[clap(long, default_value_t = 512)]
    pub iter: usize,

    /// Output file path for results
    #[clap(long, default_value = DEFAULT_OUTPUT_FILE)]
    pub output_file: Utf8PathBuf,
}
