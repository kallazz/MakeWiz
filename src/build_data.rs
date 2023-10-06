//! Handles data required for generating a Makefile.
//!
//! It extracts appropriate file names and categorizes them as source files or header files. 
//! It also generates output file names for source files (.o for C/C++ and .class for Java).

use std::fs;
use crate::StringVector;

const SOURCE_EXTENSIONS: [&str; 3] = ["c", "cpp", "java"]; 
const HEADER_EXTENSIONS: [&str; 2] = ["h", "hpp"]; 

/// A struct that holds data for generating a Makefile.
///
/// *Note: Java Makefiles use only the `source_files` and `compiled_files` fields.*
#[derive(PartialEq, Debug)]
pub struct BuildData {
    /// The name of the C/C++ compiler used for building the project.
    pub compiler: String,

    /// The name of the executable file that will be generated after compilation.
    pub executable: String,

    /// Collection of header files (.h, .hpp).
    pub header_files: StringVector,

    /// Collection of compiled files (.o for C/C++ and .class for Java).
    pub compiled_files: StringVector,

    /// Collection of source files (.c, .cpp, .java).
    pub source_files: StringVector,

    /// Additional linker flags for the project.
    pub lflags: String,

    /// Additional library flags for linking.
    pub ldlibs: String,
}

impl BuildData {
    /// Creates a new `BuildData` instance with empty strings as values.
    fn new() -> BuildData {
        BuildData { 
            compiler: String::new(),
            executable: String::new(),
            header_files: StringVector::new(),
            compiled_files: StringVector::new(),
            source_files: StringVector::new(),
            lflags: String::new(),
            ldlibs: String::new(),
        }
    }

    fn generate_compiled_files(&mut self) {
        let mut output_files: Vec<String> = Vec::new();

        for source_file in &self.source_files.0 {
            let mut new_file = source_file.clone();
            let name_len = source_file.len();

            if &source_file[name_len - 2..] == ".c" {
                new_file.truncate(name_len - 2);
                new_file.push_str(".o");
            }
            else if &source_file[name_len - 4..] == ".cpp" {
                new_file.truncate(name_len - 4);
                new_file.push_str(".o");
            }
            else {
                new_file.truncate(name_len - 5);
                new_file.push_str(".class");
            }

            output_files.push(new_file);
        }

        self.compiled_files = StringVector(output_files)
    }

    /// Extracts file names from the given directory and categorizes them
    /// into source files and header files. Also generates compiled file names(.o and .class).
    ///
    /// # Arguments
    ///
    /// * `paths` - A `ReadDir` iterator representing the directory to scan.
    /// * `allowed_extensions` - An array of allowed file extensions. Any other extensions will be ignored.
    ///
    /// # Returns
    ///
    /// A `Result` containing the extracted `BuildData` or an error.
    pub fn extract_names(paths: fs::ReadDir) -> Result<BuildData, Box<dyn std::error::Error>> {
        let mut files = BuildData::new();

        for path_result in paths {
            let path = path_result?;
            let extension = FileType::get_extension_type(&path);
            let name = path.path().file_name().unwrap().to_str().unwrap().to_string();

            match extension {
                FileType::Source => files.source_files.0.push(name),
                FileType::Header => files.header_files.0.push(name),
                FileType::Other => {}
            }
        }

        files.source_files.0.sort();
        files.header_files.0.sort();
        files.generate_compiled_files();
        Ok(files)
    }
}

enum FileType {
    Source,
    Header,
    Other
}

impl FileType {
    fn get_extension_type(file_name: &fs::DirEntry) -> FileType {
        let path = file_name.path();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_names_no_correct_files() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/no-correct-files").unwrap();
        let expected = BuildData::new();
        let result = BuildData::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_without_folders() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/standard-without-folders").unwrap();
        let expected = BuildData {
            compiler: String::new(),
            executable: String::new(),
            header_files: StringVector(vec![String::from("c_header.h"), String::from("cpp_header.hpp")]),
            source_files: StringVector(vec![String::from("c_source.c"), String::from("cpp_source.cpp")]),
            compiled_files: StringVector(vec![String::from("c_source.o"), String::from("cpp_source.o")]),
            lflags: String::new(),
            ldlibs: String::new(),
        };
        let result = BuildData::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_with_folders() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/standard-with-folders").unwrap();
        let expected = BuildData {
            compiler: String::new(),
            executable: String::new(),
            header_files: StringVector(vec![String::from("c_header.h"), String::from("cpp_header.hpp")]),
            compiled_files: StringVector(vec![String::from("c_source.o"), String::from("cpp_source.o")]),
            source_files: StringVector(vec![String::from("c_source.c"), String::from("cpp_source.cpp")]),
            lflags: String::new(),
            ldlibs: String::new(),
        };
        let result = BuildData::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }
}
