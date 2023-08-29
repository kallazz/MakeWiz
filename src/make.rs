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
        makefile.add_comment("Compiler and flags");
        makefile.add_one("CC", &file_names.compiler);
        makefile.add_multiple("FLAGS", &vec!["-g".to_string(), "-c".to_string(), "-Wall".to_string(),]);
        makefile.add_one("LFLAGS", "");
        makefile.file.push('\n');

        makefile.add_comment("Source files and object files");
        makefile.add_multiple("OBJS", &file_names.objects);
        makefile.add_multiple("SOURCE", &file_names.sources);
        makefile.add_multiple("HEADER", &file_names.headers);
        makefile.add_one("OUT", &file_names.executable);
        makefile.file.push('\n');

        makefile.add_comment("Libraries");
        makefile.add_one("LDLIBS", "");
        makefile.file.push('\n');

        makefile.add_comment("Default target");
        makefile.add_all();
        makefile.file.push('\n');

        makefile.add_comment("Linking rules");
        makefile.add_linking_rules();
        makefile.file.push('\n');

        makefile.add_comment("Compilation rules");
        makefile.add_compilation_rules();
        makefile.file.push('\n');

        makefile.add_comment("Clean rule");
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

    fn add_comment(&mut self, comment: &str) {
        let mut new_line = String::from("# ");
        new_line.push_str(&comment);

        new_line.push('\n');

        self.file.push_str(&new_line);
    }

    fn add_all(&mut self) {
        let new_line = "all: $(OUT)\n".to_string();

        self.file.push_str(&new_line);
    }

    fn add_linking_rules(&mut self) {
        let new_line = "$(OUT): $(OBJS)\n\t$(CC) -g $(OBJS) -o $(OUT) $(LFLAGS) $(LDLIBS)\n";

        self.file.push_str(&new_line);
    }

    fn add_compilation_rules(&mut self) {
        let new_line = "%.o: %.cpp $(HEADER)\n\t$(CC) $(FLAGS) -o $@ $<\n";

        self.file.push_str(&new_line);
    }

    fn add_clean(&mut self) {
        let new_line = "clean:\n\trm -f $(OBJS) $(OUT)\n";

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

        let expected = "all: $(OUT)\n";

        assert_eq!(expected, makefile.get_file());
    }

    #[test]
    fn adding_clean() {
        let mut makefile = Makefile::new();

        makefile.add_clean();

        let expected = "clean:\n\trm -f $(OBJS) $(OUT)\n";

        assert_eq!(expected, makefile.get_file());
    }
}
