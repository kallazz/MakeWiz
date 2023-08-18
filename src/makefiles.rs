
pub struct Makefile {
    file: String
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            file: String::new()
        }
    }

    pub fn add_line(&mut self, file_type: String, files: &Vec<String>) {
        let mut new_line = file_type;
        new_line.push_str(" =");
        
        for file in files {
            new_line.push(' ');
            new_line.push_str(file);
        }
        new_line.push('\n');

        self.file.push_str(&new_line);
    }

    pub fn get_file(&self) -> &str {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_lines() {
        let mut makefile = Makefile::new();
        let object_files = vec!["main.o".to_string(), "file.o".to_string(), "Car.o".to_string(), "Plane.o".to_string()];
        let source_files = vec!["main.c".to_string(), "file.c".to_string(), "Car.cpp".to_string(), "Plane.cpp".to_string()];

        makefile.add_line("OBJS".to_string(), &object_files);
        makefile.add_line("SOURCE".to_string(), &source_files);

        let expected = "OBJS = main.o file.o Car.o Plane.o\n\
            SOURCE = main.c file.c Car.cpp Plane.cpp\n";

        assert_eq!(expected, makefile.get_file());
    }
}
