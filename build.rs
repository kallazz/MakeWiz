use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

use std::env;
use std::io::Error;

include!("src/args.rs");

#[allow(dead_code)]
const SHELLS: [Shell; 3] = [Shell::Bash, Shell::Fish, Shell::Zsh];

#[allow(dead_code)]
fn generate_autocompletions() -> Result<(), Error> {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");

    let mut cmd = MakeWizArgs::command();
    for shell in SHELLS {
        generate_to(shell, &mut cmd, "makewiz", &outdir)?;
        println!("cargo:warning=autocompletion file generated for {}: {:?}", shell, outdir);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "generate_completions")]
    generate_autocompletions()?;

    Ok(())
}