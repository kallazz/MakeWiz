use genmake::make;
use genmake::files;
use genmake::args::{self, Commands};
use genmake::defaults::{self, Config};

use clap::Parser;
use directories::ProjectDirs;

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

    //Set directory where the config file will be placed
    let config_dir = ProjectDirs::from("", "",  "genmake")
        .expect("Valid home directory path for the config file couldn't be retrieved");
    let config_path = config_dir.config_dir();

    //If the directory doesn't exist, create it
    fs::create_dir_all(config_path).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
        
    let config_path= config_path.join("config.toml");
    // Linux:   /home/<username>/.config/genmake/config.toml
    // Windows: C:\Users\<username>\AppData\Roaming\genmake\config.toml
    // macOS:   /Users/<username>/Library/Application Support/genmake/config.toml

    //Initialize config
    Config::init_config(&config_path);
    let config = Config::get_current_config(&config_path);

    //Set config values to later write them to the Makefile
    file_names.compiler = config.compiler_name;
    file_names.executable = config.executable_name;

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
                Config::update_config(defaults::Attribute::CompilerName(compiler.name.clone()), &config_path);
            },

            Commands::SetExecutable(executable) => {
                Config::update_config(defaults::Attribute::ExecutableName(executable.name.clone()), &config_path);
            },

            Commands::Default => {
                Config::print_config_values(&config_path);
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
