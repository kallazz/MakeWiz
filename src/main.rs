use genmake::args;
use genmake::make;
use genmake::files;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let executable = args::parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    file_names.set_executable_file(executable.to_string());

    //Creating the makefile
}
