# MakeWiz
MakeWiz is a handy command-line tool designed to make working with Makefiles easier.
With just one simple command, MakeWiz does the work of creating a neat Makefile that perfectly fits the files in your directory. 
It is available for all major Linux distros.

[![Crates.io](https://img.shields.io/crates/v/makewiz.svg)](https://crates.io/crates/makewiz)

🔐 MakeWiz is dual-licensed under MIT or Apache 2.0.

### Quick links

* [Demonstration](#demonstration-of-makewiz-in-action)
* [Installation](#installation)
* [Shell auto-completion](#shell-auto-completion)
* [User Guide](#user-guide)
* [Your feedback](#your-feedback)

## 📺Demonstration of MakeWiz in action

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



## 🚀Installation

### Arch Linux
If you are an **Arch Linux** user(or any other Arch-based distros like **Manjaro**), you can install MakeWiz from [AUR](https://aur.archlinux.org/packages/makewiz-bin/) using a tool like `yay` or `paru`:

```
yay -S makewiz
```

### Debian
If you are a **Debian** user(or any other Debian-based distros like **Ubuntu**), you can install MakeWiz using a `.deb` file:

```
sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.7.0/makewiz_0.7.0_amd64.deb
sudo dpkg -i makewiz_0.7.0_amd64.deb
```

### RedHat/Fedora
If you are using a **RedHat-based Linux distribution** like **Fedora** or **CentOS**, you can install MakeWiz using a **.rpm** file:

```
sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.7.0/makewiz-0.7.0-1.x86_64.rpm
sudo rpm -i makewiz-0.7.0-1.x86_64.rpm
```

### Cargo
If you have **Rust** installed, you can download MakeWiz using **cargo**:

```
cargo install makewiz
```



## 💡Shell auto-completion

MakeWiz supports shell auto-completion for **Bash, Fish and Zsh**.

You can find the shell completion scripts in [MakeWiz binary releases](https://github.com/kallazz/MakeWiz/releases/) starting from `v0.7.0`.
You can also download them using these commands:

**Bash:**
```
sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.7.0/makewiz.bash
```

**Fish:**
```
sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.7.0/makewiz.fish
```


**Zsh:**
```
sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.7.0/_makewiz
```

After downloading the script for your preferred shell, follow these steps to enable auto-completion:

For **Bash:** Move the `makewiz.bash` to either `$XDG_CONFIG_HOME/bash_completion/` or `/etc/bash_completion.d/`.

For **Fish:** Move the `makewiz.fish` to `$HOME/.config/fish/completions/`.

For **Zsh:** Move the `_makewiz` to one of your `$fpath` directories. If you are unsure how to do this:
- place `_makewiz` in a directory of your choice, for example `~/.zsh/completions/`
- add `fpath=(~/.zsh/completions $fpath)` to your `~/.zshrc`, replacing `~/.zsh/completions` with your chosen directory.

After opening a new Terminal window, everything should work smoothly.



## 📖User Guide
To generate a Makefile using MakeWiz, simply enter the command `makewiz` in your terminal.

By default, MakeWiz will create a Makefile with the executable name *main* and compiler *g++*. You can change this behaviour by using commands and options listed below. If you are not sure what your default compiler and executable values are, you can just run `makewiz default`.

```
MakeWiz is a command line tool that generates a Makefile based on the files in your directory

Usage: makewiz [OPTIONS] [COMMAND]

Commands:
  set-compiler    Set the default compiler name
  set-executable  Set the default executable name
  default         Show default values
  help            Print this message or the help of the given subcommand(s)

Options:
  -c, --compiler <COMPILER_NAME>      Set the compiler name for this Makefile
  -e, --executable <EXECUTABLE_NAME>  Set the executable name for this Makefile
  -m, --math                          Add the math library(-lm) to this Makefile
  -t, --thread                        Add the thread library(-lpthread) to this Makefile
  -r, --crypto                        Add the crypto library(-lcrypto) to this Makefile
      --cunit                         Add the CUnit library(-lcunit) to this Makefile
      --cppunit                       Add the CPPUnit library(-lcppunit) to this Makefile
  -h, --help                          Print help
  -V, --version                       Print version
```



## 📣Your feedback
If you have any questions, suggestions, or run into any issues, feel free to head over to the [Issues](https://github.com/kallazz/MakeWiz/issues) tab. Your feedback is very important to me.
