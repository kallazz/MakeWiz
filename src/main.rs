use genmake::args;
use genmake::make;
use genmake::files;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (executable, compiler) = args::parse_arguments(&args).unwrap_or_else(|err| {
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

    file_names.set_executable(executable);
    file_names.set_compiler(compiler);

    //Creating the makefile
    let makefile = make::Makefile::create(&file_names);

    fs::write("./Makefile", makefile.get_file()).expect("Unable to create a Makefile");
    println!("Makefile successfully created");
}
