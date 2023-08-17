mod files_and_strings;
mod makefiles;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let executable = files_and_strings::parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files_and_strings::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    file_names.set_executable_file(executable.to_string());

    //Creating the makefile
    let mut makefile = makefiles::Makefile::new();
    makefile.add_objs(&file_names);
    makefile.print();
}
