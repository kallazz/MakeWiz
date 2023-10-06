//! This module is responsible for generating autocompletions for the MakeWiz CLI.
//!
//! It generates autocompletion scripts for Bash, Fish, and Zsh. 
//! The generated autocompletion scripts are placed in the `completions/` directory
//! within the crate's cargo manifest directory.
//! 
//! To generate the scripts, run `cargo run --bin generate_completions`.

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

use std::env;
use std::io::Error;

use makewiz::cli::CLI;

#[allow(dead_code)]
const SHELLS: [Shell; 3] = [Shell::Bash, Shell::Fish, Shell::Zsh];

#[allow(dead_code)]
fn generate_autocompletions() -> Result<(), Error> {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");

    let mut cmd = CLI::command();
    for shell in SHELLS {
        generate_to(shell, &mut cmd, "makewiz", &outdir)?;
        println!("cargo:warning=autocompletion file generated for {}: {:?}", shell, outdir);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    generate_autocompletions()?;

    Ok(())
}