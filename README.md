# MakeWiz
MakeWiz is a handy command-line tool designed to make working with Makefiles easier.
With just one simple command, MakeWiz does the work of creating a neat Makefile that perfectly fits the files in your directory. 
Right now, it is available for Debian and Arch based Linux distros, but other systems should be supported in the future too.

🔐 MakeWiz is licensed under MIT license.

### Quick links

* [Demonstration](#demonstration-of-makewiz-in-action)
* [Installation](#installation)
* [User Guide](#user-guide)
* [Your feedback](#your-feedback)

## Demonstration of MakeWiz in action

Let's say that you have a directory with such files

* `Bike.cpp` `Bike.hpp`
* `Car.cpp` `Car.hpp`
* `Vehicle.hpp`
* `main.cpp` and executable `main`

After running `makewiz`, a Makefile like this will be created

```Makefile
# Compiler and flags
CC = g++
FLAGS = -g -c -Wall
LFLAGS =

# Source files and object files
OBJS = Bike.o Car.o main.o
SOURCE = Bike.cpp Car.cpp main.cpp
HEADER = Bike.hpp Car.hpp Vehicle.hpp
OUT = main

# Libraries
LDLIBS =

# Default target
all: $(OUT)

# Linking rules
$(OUT): $(OBJS)
    $(CC) -g $(OBJS) -o $(OUT) $(LFLAGS) $(LDLIBS)

# Compilation rules
%.o: %.cpp $(HEADER)
    $(CC) $(FLAGS) -o $@ $<

# Clean rule
clean:
    rm -f $(OBJS) $(OUT)
```

All files with extensions other than `.cpp` `.hpp` `.c` `.h` will be automatically ignored by `makewiz`

## Installation

### Arch Linux
If you are an **Arch Linux** user(or any other Arch-based distros like **Manjaro**), you can install MakeWiz from [AUR](https://aur.archlinux.org/packages/makewiz-bin/) using a tool like `yay` or `paru`:

```
$ yay -S makewiz
```

### Debian
If you are a **Debian** user(or any other Debian-based distros like **Ubuntu**), you can install MakeWiz using a `.deb` file:

```
$ sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.6.1/makewiz_0.6.1_amd64.deb
$ sudo dpkg -i makewiz_0.6.1_amd64.deb
```

If you wish to install a version other than the latest, all `.deb` binaries will be available in [MakeWiz releases](https://github.com/kallazz/MakeWiz/releases/).

### Cargo
If you have **Rust** installed, you can download MakeWiz using **cargo**:

```
$ cargo install makewiz
```

## User Guide
To generate a Makefile using MakeWiz, simply enter the command `makewiz` in your terminal.

By default, MakeWiz will create a Makefile with the executable name *main* and compiler *g++*. You can change this behaviour by using commands and options listed below. If you are not sure what your default compiler and executable values are, you can just run `makewiz default`.

```
MakeWiz is a command line tool that generates a Makefile based on the files in your directory

Usage: makewiz [OPTIONS] [COMMAND]

Commands:
  set-compiler <COMPILER_NAME>      Set the default compiler name
  set-executable <EXECUTABLE_NAME>  Set the default executable name
  default                           Show default values
  help                              Print this message or the help of the given subcommand(s)

Options:
  -c, --compiler <COMPILER_NAME>      Set the compiler name for THIS Makefile
  -e, --executable <EXECUTABLE_NAME>  Set the executable name for THIS Makefile
  -h, --help                          Print help
  -V, --version                       Print version
```

## Your feedback
If you have any questions, suggestions, or run into any issues, feel free to head over to the [Issues](https://github.com/kallazz/MakeWiz/issues) tab. Your feedback is very important to me.
