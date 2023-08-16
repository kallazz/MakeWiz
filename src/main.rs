use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (directory_path, executable_file) = parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let paths_to_files = fs::read_dir(directory_path).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let files: Vec<String> = paths_to_file_names(paths_to_files).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}

fn parse_arguments(args: &Vec<String>) -> Result<(&str, &str), &str> {
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    //Flags will be added later

    let directory_path = &args[1];
    let executable_file = &args[2];

    Ok((directory_path, executable_file))
}

fn paths_to_file_names(file_paths: fs::ReadDir) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut file_names = Vec::new();

    for entry in file_paths {
        if let Some(file_name) = entry?.file_name().to_str() {
            file_names.push(file_name.to_string());
        }
    }

    Ok(file_names)
}
