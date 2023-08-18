mod files_handling;
mod makefiles;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let executable = files_handling::parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut file_names = files_handling::FileNames::extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    file_names.set_executable_file(executable.to_string());

    //Creating the makefile
    let mut makefile = makefiles::Makefile::new();
    makefile.add_line("OBJS".to_string(), file_names.get_objects());
    makefile.add_line("SOURCE".to_string(), file_names.get_sources());
    makefile.add_line("HEADER".to_string(), file_names.get_headers());
}
