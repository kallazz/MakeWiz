use makewiz::build_data;
use makewiz::cli::{self, Commands};
use makewiz::user_config::{self, UserConfig};
use clap::Parser;
use directories::ProjectDirs;

use std::fs;
use std::process;

const C_EXTENSIONS: [&str; 2] = ["c", "h"];
const CPP_EXTENSIONS: [&str; 2] = ["cpp", "hpp"];
const JAVA_EXTENSIONS: [&str; 1] = ["java"];

fn main() {
    // Set directory where the config file will be placed
    let config_dir = ProjectDirs::from("", "",  "makewiz")
        .expect("Valid home directory path for the config file couldn't be retrieved");
    let config_path = config_dir.config_dir();

    // If the directory doesn't exist, create it
    fs::create_dir_all(config_path).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    // Linux:   /home/<username>/.config/makewiz/config.toml
    // Windows: C:\Users\<username>\AppData\Roaming\makewiz\config.toml
    // macOS:   /Users/<username>/Library/Application Support/makewiz/config.toml
    let config_path= config_path.join("config.toml");
    let config = UserConfig::get_current_config(&config_path);


    // Get user arguments
    let args = cli::CLI::parse();

    // Check if both subcommand and flags are provided
    if args.subcommands_provided() && args.flags_provided() {
        eprintln!("Error: Cannot use subcommands and flags at the same time!");
        std::process::exit(1);
    }

    // Handle subcommands
    let file_extensions = [];
    match &args.command {
        Some(command) => match command {
            Commands::C => {
                let file_extensions: [&str; 2] = C_EXTENSIONS;
            },

            Commands::Cpp => {
                let file_extensions: [&str; 2] = CPP_EXTENSIONS;
            },

            Commands::Java => {
                let file_extensions: [&str; 1] = JAVA_EXTENSIONS;
            },

            Commands::SetCompiler(compiler) => {
                UserConfig::update_config(user_config::Attribute::CompilerName(compiler.name.clone()), &config_path);
            },

            Commands::SetExecutable(executable) => {
                UserConfig::update_config(user_config::Attribute::ExecutableName(executable.name.clone()), &config_path);
            },

            Commands::Default => {
                UserConfig::print_config_values(&config_path);
            }
        },
        None => { }
    }

    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = build_data::BuildData::extract_names(paths_to_files, &file_extensions).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    //Set config values to later write them to the Makefile
    file_names.compiler = config.compiler_name;
    file_names.executable = config.executable_name;

    // Handle options
    if let Some(executable) = &args.executable {
        file_names.executable = executable.clone();
    }

    if let Some(compiler) = &args.compiler {
        file_names.compiler = compiler.clone();
    }

    // Handle flags
    let (lflags, ldlibs) = args.parse_flags();
    file_names.lflags = lflags;
    file_names.ldlibs = ldlibs;

    // Don't create the Makefile for the subcommands
    if args.subcommands_provided() { std::process::exit(0); } 

    // Creating the makefile
    let makefile = makewiz::generate_makefile(&file_names);

    fs::write("./Makefile", makefile).expect("Unable to create a Makefile");
    println!("Makefile successfully created");
}
