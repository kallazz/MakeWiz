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

#[cfg(test)]
mod tests {
    use super::*;

    //Testing parse_arguments()
    #[test]
    fn parse_arguments_no_arguments() {
        let args: Vec<String> = vec![String::from("target/debug/genmake")];
        assert_eq!(Err("Not enough arguments"), parse_arguments(&args));
    }

    #[test]
    fn parse_arguments_one_argument() {
        let args: Vec<String> = vec![String::from("target/debug/genmake"), String::from("filename")];
        assert_eq!(Ok("filename"), parse_arguments(&args));
    }

    //Testing extract_names()
    #[test]
    fn extract_names_no_files() {
        let paths = fs::read_dir("./test-dirs/empty").unwrap();
        let expected_vec: Vec<String> = vec![];
        let result_vec = extract_names(paths).unwrap();

        assert_eq!(expected_vec, result_vec);
    }

    #[test]
    fn extract_names_no_correct_files() {
        let paths = fs::read_dir("./test-dirs/no-correct-files").unwrap();
        let expected_vec: Vec<String> = vec![];
        let result_vec = extract_names(paths).unwrap();

        assert_eq!(expected_vec, result_vec);
    }

    #[test]
    fn extract_names_correct_files_without_folders() {
        let paths = fs::read_dir("./test-dirs/standard-without-folders").unwrap();
        let expected_vec: Vec<String> = vec![String::from("cpp_header.hpp"), 
            String::from("c_header.h"), String::from("cpp_source.cpp"), String::from("c_source.c")];
        let result_vec = extract_names(paths).unwrap();

        assert_eq!(expected_vec, result_vec);
    }

    #[test]
    fn extract_names_correct_files_with_folders() {
        let paths = fs::read_dir("./test-dirs/standard-with-folders").unwrap();
        let expected_vec: Vec<String> = vec![String::from("cpp_header.hpp"), 
            String::from("c_header.h"), String::from("cpp_source.cpp"), String::from("c_source.c")];
        let result_vec = extract_names(paths).unwrap();

        assert_eq!(expected_vec, result_vec);
    }
}