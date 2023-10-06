#![doc = include_str!("../README.md")]

pub mod cli;
pub mod build_data;
pub mod user_config;

use std::fmt;
use crate::build_data::BuildData;

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

pub fn generate_makefile(file_names: &BuildData) -> String {
    let makefile = format!("\
# Compiler and flags
CC = {}
FLAGS = -g -c -Wall
LFLAGS = {}

# Source files and object files
OBJS = {}
SOURCE = {}
HEADER = {}
OUT = {}

# Libraries
LDLIBS = {}

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
\trm -f $(OBJS) $(OUT)\n",
file_names.compiler, file_names.lflags, file_names.compiled_files,
file_names.source_files, file_names.header_files, file_names.executable,
file_names.ldlibs);

    makefile
}

pub fn generate_java_makefile(file_names: &BuildData) -> String {
    let makefile = format!("\
# Compiler and flags
JC = javac
JFLAGS = -g

# Source files and compiled classes
SOURCE = {}
CLASSES = {}

# Default target
default: $(CLASSES)

# Compilation rule
%.class: %.java
\t$(JC) $(JFLAGS) $<

# Clean rule to remove generated .class files
clean:
\trm -f $(CLASSES)\n",
file_names.source_files, file_names.compiled_files);

    makefile
}