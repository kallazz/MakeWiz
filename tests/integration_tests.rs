use makewiz::cli;
use makewiz::build_data;

mod test {
    use super::*;

    use std::fs;
    use clap::Parser;

    #[test]
    fn makefile_creation() {
        let paths_to_files = fs::read_dir("./test-dirs/test-makefile-creation").unwrap();

        let mut file_names = build_data::BuildData::extract_names(paths_to_files, &[".c", ".cpp", ".h", ".hpp"]).unwrap();

        let args = vec![String::from("target/debug/makewiz"), String::from("-e"),
            String::from("executable"), String::from("-c"), String::from("compiler"), 
            String::from("-m"), String::from("-t"), String::from("-r"),
            String::from("--cunit"), String::from("--cppunit")];

        let parsed_args = cli::CLI::parse_from(args);

        if let Some(executable) = &parsed_args.executable {
            file_names.executable = executable.clone();
        }
        if let Some(compiler) = &parsed_args.compiler {
            file_names.compiler = compiler.clone();
        }

        let (lflags, ldlibs) = parsed_args.parse_flags();
        file_names.lflags = lflags.clone();
        file_names.ldlibs = ldlibs.clone();

        let expected = "\
# Compiler and flags
CC = compiler
FLAGS = -g -c -Wall
LFLAGS = -lpthread -lm

# Source files and object files
OBJS = AnotherClass.o SomeClass.o main.o
SOURCE = AnotherClass.cpp SomeClass.cpp main.cpp
HEADER = AnotherClass.hpp SomeClass.hpp SomeHeader.hpp
OUT = executable

# Libraries
LDLIBS = -lcunit -lcppunit -lcrypto

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
        assert_eq!(expected, makewiz::generate_makefile(&file_names));
    }
}
