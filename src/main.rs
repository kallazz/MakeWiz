use std::env;
use std::fs;
use std::fs::DirEntry;
use std::process;

const EXTENSIONS: [&str; 4] = ["cpp", "hpp", "c", "h"]; 

fn main() {
    let args: Vec<String> = env::args().collect();

    let executable_name = parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let paths_to_files = fs::read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let file_names = extract_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}

fn parse_arguments(args: &Vec<String>) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    //Flags will be added later

    let executable_name = &args[1];

    Ok(executable_name)
}

fn extract_names(paths: fs::ReadDir) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut names: Vec<String> = Vec::new();

    for path_result in paths {
        let path = path_result?;
        if check_extension(&path) {
            let name = path.path().file_name().unwrap().to_str().unwrap().to_string();
            names.push(name);
        }
    }

    Ok(names)
}

fn check_extension(name: &DirEntry) -> bool {
    let path = name.path();
    let extension = path.extension();

    match extension {
        Some(ext) => EXTENSIONS.contains(&ext.to_str().unwrap()),
        None => false
    }
}
