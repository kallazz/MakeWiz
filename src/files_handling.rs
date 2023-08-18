use std::fs;

//Constants
const SOURCE_EXTENSIONS: [&str; 2] = ["c", "cpp"]; 
const HEADER_EXTENSIONS: [&str; 2] = ["h", "hpp"]; 

//My types
#[derive(PartialEq)]
#[derive(Debug)]
pub struct FileNames {
    sources: Vec<String>,
    objects: Vec<String>,
    headers: Vec<String>,
    executable: String
}

impl FileNames {
    fn new() -> FileNames {
        FileNames { 
            sources: Vec::new(),
            objects: Vec::new(),
            headers: Vec::new(),
            executable: String::new()
        }
    }

    //Setters
    fn add_source_file(&mut self, name: String) {
        self.sources.push(name)
    }

    fn add_header_file(&mut self, name: String) {
        self.headers.push(name)
    }

    pub fn set_executable_file(&mut self, name: String) {
        self.executable = name;
    }

    //Getters
    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn get_objects(&self) -> &Vec<String> {
        &self.objects
    }

    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    fn sort_source_files(&mut self) {
        self.sources.sort()
    }

    fn sort_header_files(&mut self) {
        self.headers.sort()
    }

    fn generate_output_files(&mut self) {
        let mut output_files: Vec<String> = Vec::new();

        for source_file in &self.sources {
            let mut new_file = source_file.clone();
            let name_len = source_file.len();

            if &source_file[name_len - 2..] == ".c" {
                //.c file
                new_file.truncate(name_len - 2);
            }
            else {
                //.cpp file
                new_file.truncate(name_len - 4);
            }

            new_file.push_str(".o");
            output_files.push(new_file)
        }

        self.objects = output_files
    }

    pub fn extract_names(paths: fs::ReadDir) -> Result<FileNames, Box<dyn std::error::Error>>{
        let mut files = FileNames::new();

        for path_result in paths {
            let path = path_result?;
            let extension = FileType::get_extension_type(&path);
            let name = path.path().file_name().unwrap().to_str().unwrap().to_string();

            match extension {
                FileType::Source => files.add_source_file(name),
                FileType::Header => files.add_header_file(name),
                FileType::Other => {}
            }
        }

        files.sort_source_files();
        files.sort_header_files();
        files.generate_output_files();
        Ok(files)
    }
}

enum FileType {
    Source,
    Header,
    Other
}

impl FileType {
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
}

pub fn parse_arguments(args: &Vec<String>) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    //Flags will be added later

    let executable_name = &args[1];

    Ok(executable_name)
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

    //Testing extract_names() + FileNames
    #[test]
    fn extract_names_no_files() {
        let paths = fs::read_dir("./test-dirs/empty").unwrap();
        let expected = FileNames::new();
        let result = FileNames::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_no_correct_files() {
        let paths = fs::read_dir("./test-dirs/no-correct-files").unwrap();
        let expected = FileNames::new();
        let result = FileNames::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_without_folders() {
        let paths = fs::read_dir("./test-dirs/standard-without-folders").unwrap();
        let expected = FileNames {
            sources: vec![String::from("c_source.c"), String::from("cpp_source.cpp")],
            objects: vec![String::from("c_source.o"), String::from("cpp_source.o")],
            headers: vec![String::from("c_header.h"), String::from("cpp_header.hpp")],
            executable: String::new()
        };
        let result = FileNames::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_with_folders() {
        let paths = fs::read_dir("./test-dirs/standard-with-folders").unwrap();
        let expected = FileNames {
            sources: vec![String::from("c_source.c"), String::from("cpp_source.cpp")],
            objects: vec![String::from("c_source.o"), String::from("cpp_source.o")],
            headers: vec![String::from("c_header.h"), String::from("cpp_header.hpp")],
            executable: String::new()
        };
        let result = FileNames::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn files_struct_set_executable_file() {
        let expected = FileNames {
            sources: vec![],
            objects: vec![],
            headers: vec![],
            executable: String::from("Name")
        };
        let mut result = FileNames::new();
        result.set_executable_file(String::from("Name"));

        assert_eq!(expected, result);
    }
}
