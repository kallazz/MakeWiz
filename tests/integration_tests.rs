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
OBJS = AnotherClass.o SomeClass.o main.o
SOURCE = AnotherClass.cpp SomeClass.cpp main.cpp
HEADER = AnotherClass.hpp SomeClass.hpp SomeHeader.hpp
OUT = executable
CC = compiler
FLAGS = -g -c -Wall
LFLAGS = 

all: $(OBJS)
\t$(CC) -g $(OBJS) -o $(OUT) $(LFLAGS)

AnotherClass.o: AnotherClass.cpp
\t$(CC) $(FLAGS) AnotherClass.cpp

SomeClass.o: SomeClass.cpp
\t$(CC) $(FLAGS) SomeClass.cpp

main.o: main.cpp
\t$(CC) $(FLAGS) main.cpp


clean:
\trm -f $(OBJS) $(OUT)\n";

        assert_eq!(expected, makefile.get_file());
    }
}