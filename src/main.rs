use makewiz::build_config;
use makewiz::args::{self, Commands};
use makewiz::user_config_manager::{self, UserConfig};

use clap::Parser;
use directories::ProjectDirs;

use std::fs;
use std::process;

fn main() {
    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = build_config::ProjectBuildConfig::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    //Set directory where the config file will be placed
    let config_dir = ProjectDirs::from("", "",  "makewiz")
        .expect("Valid home directory path for the config file couldn't be retrieved");
    let config_path = config_dir.config_dir();

    //If the directory doesn't exist, create it
    fs::create_dir_all(config_path).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
        
    let config_path= config_path.join("config.toml");
    // Linux:   /home/<username>/.config/makewiz/config.toml
    // Windows: C:\Users\<username>\AppData\Roaming\makewiz\config.toml
    // macOS:   /Users/<username>/Library/Application Support/makewiz/config.toml

    //Initialize config
    UserConfig::init_config(&config_path);
    let config = UserConfig::get_current_config(&config_path);

    //Set config values to later write them to the Makefile
    file_names.compiler = config.compiler_name;
    file_names.executable = config.executable_name;

    //Get user arguments
    let args = args::MakeWizArgs::parse();

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
                UserConfig::update_config(user_config_manager::Attribute::CompilerName(compiler.name.clone()), &config_path);
            },

            Commands::SetExecutable(executable) => {
                UserConfig::update_config(user_config_manager::Attribute::ExecutableName(executable.name.clone()), &config_path);
            },

            Commands::Default => {
                UserConfig::print_config_values(&config_path);
            }
        },
        None => { }
    }

    //Don't create the Makefile for the subcommands
    if args.subcommands_provided() { std::process::exit(0); } 

    //Creating the makefile
    let makefile = makewiz::generate_makefile(&file_names);

    fs::write("./Makefile", makefile).expect("Unable to create a Makefile");
    println!("Makefile successfully created");
}
