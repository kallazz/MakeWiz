pub mod cli;
pub mod build_config;
pub mod user_config_manager;

pub fn generate_makefile(file_names: &build_config::ProjectBuildConfig) -> String {
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
file_names.compiler, file_names.lflags, file_names.objects,
file_names.sources, file_names.headers, file_names.executable,
file_names.ldlibs);

    makefile
}
