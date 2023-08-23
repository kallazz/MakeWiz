use genmake::args;
use genmake::args::set_default_env_var;
use genmake::make;
use genmake::files;

use args::Commands;
use clap::Parser;
use std::fs;
use std::process;
use std::env;

fn main() {
    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    //Set default values for env variables if they are empty
    set_default_env_var("COMPILER", "g++");
    set_default_env_var("EXECUTABLE", "main");

    file_names.compiler = env::var("COMPILER").unwrap();
    file_names.executable = env::var("EXECUTABLE").unwrap();

    //User arguments
    let args = args::GenmakeArgs::parse();

    // Check if both subcommand and flags are provided
    if args.subcommands_provided() && args.flags_provided() {
        eprintln!("Error: Cannot use subcommands and flags at the same time!");
        std::process::exit(1);
    }

    //Handling flags
    if let Some(executable) = &args.executable {
        file_names.executable = executable.clone();
    }

    if let Some(compiler) = &args.compiler {
        file_names.compiler = compiler.clone();
    }

    //Handling subcommands
    match &args.command {
        Some(command) => match command {
            Commands::SetCompiler(compiler) => {
                env::set_var("COMPILER", compiler.name.clone());
                file_names.compiler = env::var("COMPILER").unwrap();
            },

            Commands::SetExecutable(executable) => {
                env::set_var("EXECUTABLE", executable.name.clone());
                file_names.executable = env::var("EXECUTABLE").unwrap();
            },

            Commands::Default => {
                println!("Default compiler name: {}", env::var("COMPILER").unwrap());
                println!("Default executable name: {}", env::var("EXECUTABLE").unwrap());
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
