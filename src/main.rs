use genmake::args;
use genmake::make;
use genmake::files;

use std::fs;
use std::process;
use args::Commands;
use clap::Parser;

fn main() {
    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    //User arguments
    let args = args::GenmakeArgs::parse();

    // Check if both subcommand and flags are provided
    if args.subcommands_provided() && args.flags_provided() {
        eprintln!("Error: Cannot use subcommands and flags at the same time!");
        std::process::exit(1);
    }

    //Handling flags
    if let Some(executable) = args.executable {
        file_names.set_executable(executable);
    }

    if let Some(compiler) = args.compiler {
        file_names.set_compiler(compiler);
    }

    //Handling subcommands
    match args.command {
        Some(command) => match command {
            Commands::SetExecutable(set_executable_args) => {
                let executable = set_executable_args.name;
                //TODO
            }

            Commands::SetCompiler(set_compiler_args) => {
                let compiler = set_compiler_args.name;
                //TODO
            }
        },
        None => { }
    }

    //Creating the makefile
    let makefile = make::Makefile::create(&file_names);

    fs::write("./Makefile", makefile.get_file()).expect("Unable to create a Makefile");
    println!("Makefile successfully created");
}
