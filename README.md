# genmake
genmake is a handy command-line tool designed to make working with Makefiles easier.
With just one simple command, genmake does the work of creating a neat Makefile that perfectly fits the files in your directory. 
Right now, it is available for Debian-based Linux distros, but other systems should be supported in the future too.
Binary downloads will be available for [every release](https://github.com/kallazz/genmake/releases/).

🔐 genmake is licensed under MIT license.

### Documentation quick links

* [Demonstration](#demonstration-of-genmake-in-action)
* [Installation](#installation)
* [User Guide](#user-guide)

### Demonstration of genmake in action

Let's say that you have a directory with such files

* `Bike.cpp` `Bike.hpp`
* `Car.cpp` `Car.hpp`
* `Vehicle.hpp`
* `main.cpp` and executable `main`

After running `genmake`, a Makefile like this will be created

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

All files with extensions other than `.cpp`, `.hpp`, `.c`, `.h` will be automatically ignored by `genmake`

### Installation
If you are a **Debian** user(or any other Debian-based distros like **Ubuntu**), you can install genmake using a `.deb` file. 

```
$ sudo curl -LO https://github.com/kallazz/genmake/releases/download/v0.5.0/genmake_0.5.0_amd64.deb
$ sudo dpkg -i genmake_0.5.0_amd64.deb
```

If you wish to install a version other than the latest, all `.deb` binaries will be available in [genmake releases](https://github.com/kallazz/genmake/releases/).

Right now **other systems are not supported**, but they will be in the future.

### User Guide
Currently, there are 3 ways to use `genmake`. Each of them will generate a Makefile.
The only difference will be the names of the executable file and the compiler.

* **No arguments:** `genmake`

   Makefile will be created with the **default compiler and executable names**. Default names are **main** for the executable and **g++** for the compiler.

   ```
   OUT = main
   CC = g++
   ```

* **One argument:** `genmake your_executable`

   Makefile will be created with the **default compiler**, but **your own executable name**.

   ```
   OUT = your_executable
   CC = g++
   ```

* **Two arguments:** `genmake your_executable your_compiler`

   Makefile will be created with **your own executable and compiler names**.

   ```
   OUT = your_executable
   CC = your_compiler
   ```

This is not a very convenient approach, if you don't want to use the default values. 
It will be modified with flags and environmental variables in the future.
