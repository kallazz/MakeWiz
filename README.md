# MakeWiz
MakeWiz is a handy command-line tool designed to make working with Makefiles easier.
With just one simple command, genmake does the work of creating a neat Makefile that perfectly fits the files in your directory. 
Right now, it is available for Debian-based Linux distros, but other systems should be supported in the future too.
Binary downloads will be available for [every release](https://github.com/kallazz/genmake/releases/).

🔐 MakeWiz is licensed under MIT license.

### Quick links

* [Demonstration](#demonstration-of-MakeWiz-in-action)
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
OBJS = Bike.o Car.o main.o
SOURCE = Bike.cpp Car.cpp main.cpp
HEADER = Bike.hpp Car.hpp Vehicle.hpp
OUT = main
CC = g++
FLAGS = -g -c -Wall
LFLAGS = 

all: $(OBJS)
    $(CC) -g $(OBJS) -o $(OUT) $(LFLAGS)

Bike.o: Bike.cpp
    $(CC) $(FLAGS) Bike.cpp

Car.o: Car.cpp
    $(CC) $(FLAGS) Car.cpp

main.o: main.cpp
    $(CC) $(FLAGS) main.cpp


clean:
    rm -f $(OBJS) $(OUT)
```

All files with extensions other than `.cpp` `.hpp` `.c` `.h` will be automatically ignored by `makewiz`

## Installation
If you are a **Debian** user(or any other Debian-based distros like **Ubuntu**), you can install MakeWiz using a `.deb` file. 

```
$ sudo curl -LO https://github.com/kallazz/MakeWiz/releases/download/v0.5.0/genmake_0.5.0_amd64.deb
$ sudo dpkg -i genmake_0.5.0_amd64.deb
```

If you wish to install a version other than the latest, all `.deb` binaries will be available in [MakeWiz releases](https://github.com/kallazz/MakeWiz/releases/).

Right now **other systems are not supported**, but they will be in the future.

## User Guide


## Your feedback
If you have any questions, suggestions, or run into any issues, feel free to head over to the [Issues](https://github.com/kallazz/genmake/issues) tab. Your feedback is very important to me.
