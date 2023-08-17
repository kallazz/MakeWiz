use std::fs;

//Constants
const SOURCE_EXTENSIONS: [&str; 2] = ["c", "cpp"]; 
const HEADER_EXTENSIONS: [&str; 2] = ["h", "hpp"]; 

//My types
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Files {
    sources: Vec<String>,
    headers: Vec<String>,
    executable: String
}

impl Files {
    fn new() -> Files {
        Files { 
            sources: Vec::new(),
            headers: Vec::new(),
            executable: String::new()
        }
    }

    fn add_source_file(&mut self, name: String) {
        self.sources.push(name)
    }

    fn add_header_file(&mut self, name: String) {
        self.headers.push(name)
    }

    pub fn set_executable_file(&mut self, name: String) {
        self.executable = name;
    }

    fn sort_source_files(&mut self) {
        self.sources.sort()
    }

    fn sort_header_files(&mut self) {
        self.headers.sort()
    }

    pub fn generate_output_files(&self) -> Vec<String> {
        let mut output_files: Vec<String> = Vec::new();

        for source_file in &self.sources {
            let name_len = source_file.len();
            if &source_file[name_len - 2..] == ".c" {
                //.c file
                let mut new_file = source_file.clone();
                new_file.truncate(name_len - 2);
                new_file.push_str(".o");
                output_files.push(new_file)
            }
            else {
                //.cpp file
                let mut new_file = source_file.clone();
                new_file.truncate(name_len - 4);
                new_file.push_str(".o");
                output_files.push(new_file)
            }
        }

        output_files
    }
}

enum FileType {
    Source,
    Header,
    Other
}

//Functions
pub fn parse_arguments(args: &Vec<String>) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    //Flags will be added later

    let executable_name = &args[1];

    Ok(executable_name)
}

pub fn extract_names(paths: fs::ReadDir) -> Result<Files, Box<dyn std::error::Error>>{
    let mut files = Files::new();

    for path_result in paths {
        let path = path_result?;
        let extension = get_extension_type(&path);
        let name = path.path().file_name().unwrap().to_str().unwrap().to_string();

        match extension {
            FileType::Source => files.add_source_file(name),
            FileType::Header => files.add_header_file(name),
            FileType::Other => {}
        }
    }

    files.sort_source_files();
    files.sort_header_files();
    Ok(files)
}

fn get_extension_type(name: &fs::DirEntry) -> FileType {
    let path = name.path();
    let extension = path.extension();

    match extension {
        Some(ext) => {
            let ext = &ext.to_str().unwrap();

            if SOURCE_EXTENSIONS.contains(ext) { FileType::Source }
            else if HEADER_EXTENSIONS.contains(ext) { FileType::Header }
            else { FileType::Other }
        },
        None => FileType::Other
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

    //Testing extract_names() + Files
    #[test]
    fn extract_names_no_files() {
        let paths = fs::read_dir("./test-dirs/empty").unwrap();
        let expected = Files::new();
        let result = extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_no_correct_files() {
        let paths = fs::read_dir("./test-dirs/no-correct-files").unwrap();
        let expected = Files::new();
        let result = extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_without_folders() {
        let paths = fs::read_dir("./test-dirs/standard-without-folders").unwrap();
        let expected = Files {
            sources: vec![String::from("c_source.c"), String::from("cpp_source.cpp")],
            headers: vec![String::from("c_header.h"), String::from("cpp_header.hpp")],
            executable: String::new()
        };
        let result = extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_with_folders() {
        let paths = fs::read_dir("./test-dirs/standard-with-folders").unwrap();
        let expected = Files {
            sources: vec![String::from("c_source.c"), String::from("cpp_source.cpp")],
            headers: vec![String::from("c_header.h"), String::from("cpp_header.hpp")],
            executable: String::new()
        };
        let result = extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn files_struct_set_executable_file() {
        let expected = Files {
            sources: vec![],
            headers: vec![],
            executable: String::from("Name")
        };
        let mut result = Files::new();
        result.set_executable_file(String::from("Name"));

        assert_eq!(expected, result);
    }

    #[test]
    fn files_struct_generate_output_files() {
        let c_files = Files {
            sources: vec!["main.c".to_string(), "someClass.c".to_string(), "anotherClass.c".to_string()],
            headers: vec![],
            executable: String::new()
        };
        let cpp_files = Files {
            sources: vec!["main.cpp".to_string(), "someClass.cpp".to_string(), "anotherClass.cpp".to_string()],
            headers: vec![],
            executable: String::new()
        };
        let expected = vec!["main.o".to_string(), "someClass.o".to_string(), "anotherClass.o".to_string()];

        assert_eq!(expected, c_files.generate_output_files());
        assert_eq!(expected, cpp_files.generate_output_files());
    }
}