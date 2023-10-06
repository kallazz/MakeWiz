//! MakeWiz is a command line tool that generates a Makefile based on the files in your directory.
//!
//! Makefiles can be generated for C, C++ and Java.

pub mod cli;
pub mod build_data;
pub mod user_config;

use std::fmt;
use crate::build_data::BuildData;

/// Represents a vector of strings.
#[derive(PartialEq, Debug)]
pub struct StringVector(Vec<String>);

impl StringVector {
    /// Creates a new empty `StringVector`.
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

/// Generates a Makefile for a C/C++ project based on the provided `BuildData`.
///
/// # Arguments
///
/// * `file_names` - A `BuildData` struct containing file names and compiler options.
///
/// # Returns
///
/// A `String` containing the generated Makefile.
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

/// Generates a Makefile for a Java project based on the provided `BuildData`.
///
/// # Arguments
///
/// * `file_names` - A `BuildData` struct containing relevant file names.
///
/// # Returns
///
/// A `String` containing the generated Makefile.
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