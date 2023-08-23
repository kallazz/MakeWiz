use genmake::args;
use genmake::make;
use genmake::files;
use genmake::defaults;

use args::Commands;
use defaults::Config;
use clap::Parser;
use std::fs;
use std::process;

fn main() {
    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    //Initialize config
    Config::init_config();
    file_names.compiler = Config::get_current_compiler();
    file_names.executable = Config::get_current_executable();

    //Get user arguments
    let args = args::GenmakeArgs::parse();

    // Check if both subcommand and flags are provided
    if args.subcommands_provided() && args.flags_provided() {
        eprintln!("Error: Cannot use subcommands and flags at the same time!");
        std::process::exit(1);
    }

    //Handle flags
    if let Some(executable) = &args.executable {
        file_names.executable = executable.clone();
    }

    if let Some(compiler) = &args.compiler {
        file_names.compiler = compiler.clone();
    }

    //Handle subcommands
    match &args.command {
        Some(command) => match command {
            Commands::SetCompiler(compiler) => {
                Config::update_compiler(&compiler.name);
            },

            Commands::SetExecutable(executable) => {
                Config::update_executable(&executable.name);
            },

            Commands::Default => {
                println!("Default compiler name: {}", Config::get_current_compiler());
                println!("Default executable name: {}", Config::get_current_executable());
            }
        },
        None => { }
    }

    //Don't create the Makefile for the subcommands
    if args.subcommands_provided() { std::process::exit(0); } 

    //Creating the makefile
    let makefile = make::Makefile::create(&file_names);

    fs::write("./Makefile", makefile.get_file()).expect("Unable to create a Makefile");
    println!("Makefile successfully created");
}
