use std::fs;
use std::fmt;

const SOURCE_EXTENSIONS: [&str; 2] = ["c", "cpp"]; 
const HEADER_EXTENSIONS: [&str; 2] = ["h", "hpp"]; 


#[derive(PartialEq, Debug)]
pub struct StringVector(Vec<String>);

impl StringVector {
    pub fn new() -> StringVector {
        StringVector(Vec::new())
    }
}

impl fmt::Display for StringVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))?;
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct ProjectBuildConfig {
    pub compiler: String,
    pub executable: String,
    pub headers: StringVector,
    pub objects: StringVector,
    pub sources: StringVector,
    pub lflags: StringVector,
    pub ldlibs: StringVector,
}

impl ProjectBuildConfig {
    fn new() -> ProjectBuildConfig {
        ProjectBuildConfig { 
            sources: StringVector::new(),
            objects: StringVector::new(),
            headers: StringVector::new(),
            executable: String::new(),
            compiler: String::new(),
            lflags: StringVector::new(),
            ldlibs: StringVector::new(),
        }
    }

    fn generate_output_files(&mut self) {
        let mut output_files: Vec<String> = Vec::new();

        for source_file in &self.sources.0 {
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

        self.objects = StringVector(output_files)
    }

    pub fn extract_names(paths: fs::ReadDir) -> Result<ProjectBuildConfig, Box<dyn std::error::Error>> {
        let mut files = ProjectBuildConfig::new();

        for path_result in paths {
            let path = path_result?;
            let extension = FileType::get_extension_type(&path);
            let name = path.path().file_name().unwrap().to_str().unwrap().to_string();

            match extension {
                FileType::Source => files.sources.0.push(name),
                FileType::Header => files.headers.0.push(name),
                FileType::Other => {}
            }
        }

        files.sources.0.sort();
        files.headers.0.sort();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "empty directory can't be added to git"]
    fn extract_names_no_files() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/empty").unwrap();
        let expected = ProjectBuildConfig::new();
        let result = ProjectBuildConfig::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_no_correct_files() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/no-correct-files").unwrap();
        let expected = ProjectBuildConfig::new();
        let result = ProjectBuildConfig::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_without_folders() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/standard-without-folders").unwrap();
        let expected = ProjectBuildConfig {
            sources: StringVector(vec![String::from("c_source.c"), String::from("cpp_source.cpp")]),
            objects: StringVector(vec![String::from("c_source.o"), String::from("cpp_source.o")]),
            headers: StringVector(vec![String::from("c_header.h"), String::from("cpp_header.hpp")]),
            executable: String::new(),
            compiler: String::new(),
            lflags: StringVector::new(),
            ldlibs: StringVector::new(),
        };
        let result = ProjectBuildConfig::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn extract_names_correct_files_with_folders() {
        let paths = fs::read_dir("./test-dirs/test-extracting-filenames/standard-with-folders").unwrap();
        let expected = ProjectBuildConfig {
            sources: StringVector(vec![String::from("c_source.c"), String::from("cpp_source.cpp")]),
            objects: StringVector(vec![String::from("c_source.o"), String::from("cpp_source.o")]),
            headers: StringVector(vec![String::from("c_header.h"), String::from("cpp_header.hpp")]),
            executable: String::new(),
            compiler: String::new(),
            lflags: StringVector::new(),
            ldlibs: StringVector::new(),
        };
        let result = ProjectBuildConfig::extract_names(paths).unwrap();

        assert_eq!(expected, result);
    }
}
