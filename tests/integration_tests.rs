use makewiz::args;
use makewiz::make;
use makewiz::files;

mod test {
    use super::*;

    use std::fs;
    use clap::Parser;

    #[test]
    fn makefile_creation() {
        let paths_to_files = fs::read_dir("./test-dirs/test-makefile-creation").unwrap();

        let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap();

        let args = vec![String::from("target/debug/makewiz"), String::from("-e"),
            String::from("executable"), String::from("-c"), String::from("compiler")];

        let parsed_args = args::MakeWizArgs::parse_from(args);
        file_names.executable = parsed_args.executable.unwrap();
        file_names.compiler = parsed_args.compiler.unwrap();

        let makefile = make::Makefile::create(&file_names);

        let expected = "\
# Compiler and flags
CC = compiler
FLAGS = -g -c -Wall
LFLAGS = 

# Source files and object files
OBJS = AnotherClass.o SomeClass.o main.o
SOURCE = AnotherClass.cpp SomeClass.cpp main.cpp
HEADER = AnotherClass.hpp SomeClass.hpp SomeHeader.hpp
OUT = executable

# Libraries
LDLIBS = 

# Default target
all: $(OUT)

# Linking rules
$(OUT): $(OBJS)
\t$(CC) -g $(OBJS) -o $(OUT) $(LFLAGS) $(LDLIBS)

# Compilation rules
%.o: %.cpp $(HEADER)
\t$(CC) $(FLAGS) -o $@ $<

# Clean rule
clean:
\trm -f $(OBJS) $(OUT)\n";
        assert_eq!(expected, makefile.get_file());
    }
}