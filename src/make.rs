use crate::files::FileNames;

pub struct Makefile {
    file: String
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            file: String::new()
        }
    }

    pub fn create(file_names: &FileNames) -> Makefile {
        let mut makefile = Makefile::new();
        makefile.add_multiple("OBJS", file_names.get_objects());
        makefile.add_multiple("SOURCE", file_names.get_sources());
        makefile.add_multiple("HEADER", file_names.get_headers());
        makefile.add_one("OUT", file_names.get_executable());
        makefile.add_one("CC", file_names.get_compiler()); //For now
        makefile.add_multiple("FLAGS", &vec!["-g".to_string(), "-c".to_string(), "-Wall".to_string(),]);
        makefile.add_one("LFLAGS", "");

        makefile.add_all();
        makefile.add_execution(file_names.get_objects(), file_names.get_sources());

        makefile.add_clean();

        makefile
    }

    fn add_multiple(&mut self, file_type: &str, items: &Vec<String>) {
        let mut new_line = file_type.to_string();
        new_line.push_str(" =");

        for item in items {
            new_line.push(' ');
            new_line.push_str(item);
        }
        new_line.push('\n');

        self.file.push_str(&new_line);
    }

    fn add_one(&mut self, file_type: &str, item: &str) {
        let mut new_line = file_type.to_string();
        new_line.push_str(" = ");
        new_line.push_str(&item);

        new_line.push('\n');

        self.file.push_str(&new_line);
    }

    fn add_all(&mut self) {
        let mut new_line = "\nall: $(OBJS)\n".to_string();
        new_line.push_str("    $(CC) -g $(OBJS) -o $(OUT) $(LFLAGS)\n");

        self.file.push_str(&new_line);
    }

    fn add_execution(&mut self, object_files: &Vec<String>, source_files: &Vec<String>) {
        let size = object_files.len();

        for i in 0..size {
            let mut new_line = String::from("\n");
            new_line.push_str(&object_files[i]);
            new_line.push_str(": ");
            new_line.push_str(&source_files[i]);
            new_line.push('\n');

            new_line.push_str("    $(CC) $(FLAGS) ");
            new_line.push_str(&source_files[i]);
            new_line.push('\n');

            self.file.push_str(&new_line);
        }
    }

    fn add_clean(&mut self) {
        let new_line = "\n\nclean:\n    rm -f $(OBJS) $(OUT)\n";

        self.file.push_str(new_line);
    }

    pub fn get_file(&self) -> &str {
        &self.file
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adding_multiple() {
        let mut makefile = Makefile::new();
        let object_files = vec!["main.o".to_string(), "file.o".to_string(), "Car.o".to_string(), "Plane.o".to_string()];
        let source_files = vec!["main.c".to_string(), "file.c".to_string(), "Car.cpp".to_string(), "Plane.cpp".to_string()];

        makefile.add_multiple("OBJS", &object_files);
        makefile.add_multiple("SOURCE", &source_files);

        let expected = "OBJS = main.o file.o Car.o Plane.o\n\
            SOURCE = main.c file.c Car.cpp Plane.cpp\n";

        assert_eq!(expected, makefile.get_file());
    }

    #[test]
    fn adding_one() {
        let mut makefile = Makefile::new();
        let executable = "main".to_string();
        let compiler = "g++".to_string(); 

        makefile.add_one("OUT", &executable);
        makefile.add_one("CC", &compiler);

        let expected = "OUT = main\n\
            CC = g++\n";

        assert_eq!(expected, makefile.get_file());
    }

    #[test]
    fn adding_all() {
        let mut makefile = Makefile::new();

        makefile.add_all();

        let expected = "\nall: $(OBJS)\n    $(CC) -g $(OBJS) -o $(OUT) $(LFLAGS)\n";

        assert_eq!(expected, makefile.get_file());
    }

    #[test]
    fn adding_execution() {
        let mut makefile = Makefile::new();
        let object_files = vec!["file.o".to_string(), "main.o".to_string(), "Car.o".to_string(), "Plane.o".to_string()];
        let source_files = vec!["file.cpp".to_string(), "main.cpp".to_string(), "Car.cpp".to_string(), "Plane.cpp".to_string()];

        makefile.add_execution(&object_files, &source_files);

        let expected = "\nfile.o: file.cpp\n    $(CC) $(FLAGS) file.cpp\n\
            \nmain.o: main.cpp\n    $(CC) $(FLAGS) main.cpp\n\
            \nCar.o: Car.cpp\n    $(CC) $(FLAGS) Car.cpp\n\
            \nPlane.o: Plane.cpp\n    $(CC) $(FLAGS) Plane.cpp\n";

        assert_eq!(expected, makefile.get_file());
    }

    #[test]
    fn adding_clean() {
        let mut makefile = Makefile::new();

        makefile.add_clean();

        let expected = "\n\nclean:\n    rm -f $(OBJS) $(OUT)\n";

        assert_eq!(expected, makefile.get_file());
    }
}
