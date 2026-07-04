//! The `wickra-shazam` reference CLI.
//!
//! Loads a `FingerprintSpec` and an asset's history from CSV, builds a
//! fingerprint index through `shazam-core`, matches the current state against
//! it, and prints the top matches as text or JSON.

mod args;
mod run;

use args::Args;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("wickra-shazam: {err}");
            ExitCode::FAILURE
        }
    }
}
