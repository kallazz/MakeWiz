use std::fs;

const EXTENSIONS: [&str; 4] = ["cpp", "hpp", "c", "h"]; 

pub fn parse_arguments(args: &Vec<String>) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    //Flags will be added later

    let executable_name = &args[1];

    Ok(executable_name)
}

pub fn extract_names(paths: fs::ReadDir) -> Result<Vec<String>, Box<dyn std::error::Error>>{
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

fn check_extension(name: &fs::DirEntry) -> bool {
    let path = name.path();
    let extension = path.extension();

    match extension {
        Some(ext) => EXTENSIONS.contains(&ext.to_str().unwrap()),
        None => false
    }
}
