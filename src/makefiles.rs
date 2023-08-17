use crate::files_and_strings::Files;

pub struct Makefile {
    file: String
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            file: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.file.push_str(text);
    }

    pub fn add_objs(&mut self, files: &Files) {
        let mut objs = "OBJS =".to_string();
        let output_files = files.generate_output_files();

        for file in output_files {
            objs.push(' ');
            objs.push_str(&file);
        }
        objs.push('\n');

        self.add_text(&objs);
    }

    pub fn print(&self) {
        println!("{}", self.file);
    }
}