mod args;
mod make;
mod files;

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
    let mut makefile = make::Makefile::new();
    makefile.add_multiple("OBJS".to_string(), file_names.get_objects());
    makefile.add_multiple("SOURCE".to_string(), file_names.get_sources());
    makefile.add_multiple("HEADER".to_string(), file_names.get_headers());
    makefile.add_one("OUT".to_string(), executable.to_string());
    makefile.add_one("CC".to_string(), "g++".to_string()); //For now
    makefile.add_multiple("FLAGS".to_string(), &vec!["-g".to_string(), "-c".to_string(), "-Wall".to_string(),]);
    makefile.add_one("LFLAGS".to_string(), "".to_string());

    makefile.add_all();
    makefile.add_execution(file_names.get_objects(), file_names.get_sources());

    makefile.add_clean();

    fs::write("./Makefile", makefile.get_file()).expect("Unable to write file");
}
