use genmake::args;
use genmake::make;
use genmake::files;

use std::fs;

#[test]
fn makefile_creation() {
    let args = vec![String::from("target/debug/genmake"), 
        String::from("executable"), String::from("compiler")];
    let (executable, compiler) = args::parse_arguments(&args).unwrap();

    let paths_to_files = fs::read_dir("./test-dirs/test-makefile-creation").unwrap();

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap();
    file_names.set_executable(executable);
    file_names.set_compiler(compiler);

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
    $(CC) -g $(OBJS) -o $(OUT) $(LFLAGS)

AnotherClass.o: AnotherClass.cpp
    $(CC) $(FLAGS) AnotherClass.cpp

SomeClass.o: SomeClass.cpp
    $(CC) $(FLAGS) SomeClass.cpp

main.o: main.cpp
    $(CC) $(FLAGS) main.cpp


clean:
    rm -f $(OBJS) $(OUT)\n";

    assert_eq!(expected, makefile.get_file());
}