use genmake::make;
use genmake::files;

use std::fs;

#[test]
fn makefile_creation() {
    let paths_to_files = fs::read_dir("./test-dirs/test-makefile-creation").unwrap();

    let mut file_names = files::FileNames::extract_names(paths_to_files).unwrap();
    file_names.set_executable_file("main".to_string());

    let makefile = make::Makefile::create(&file_names);

    let expected = "\
OBJS = AnotherClass.o SomeClass.o main.o
SOURCE = AnotherClass.cpp SomeClass.cpp main.cpp
HEADER = AnotherClass.hpp SomeClass.hpp SomeHeader.hpp
OUT = main
CC = g++
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