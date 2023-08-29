use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about)]
pub struct MakeWizArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set the compiler name for this Makefile
    #[arg(short, long, value_name = "COMPILER_NAME")]
    pub compiler: Option<String>,

    /// Set the executable name for this Makefile
    #[arg(short, long, value_name = "EXECUTABLE_NAME")]
    pub executable: Option<String>,

    /// Add the math library(-lm) to this Makefile
    #[arg(short, long)]
    math: bool,

    /// Add the thread library(-lpthread) to this Makefile
    #[arg(short, long)]
    thread: bool,

    /// Add the crypto library(-lcrypto) to this Makefile
    #[arg(short = 'r', long)]
    crypto: bool,

    /// Add the CUnit library(-lcunit) to this Makefile
    #[arg(long)]
    cunit: bool,

    /// Add the CPPUnit library(-lcppunit) to this Makefile
    #[arg(long)]
    cppunit: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set the default compiler name
    SetCompiler(NameArgument),

    /// Set the default executable name
    SetExecutable(NameArgument),

    /// Show default values
    Default,
}

#[derive(Args)]
#[group(required = true)]
pub struct NameArgument {
    pub name: String,
}

impl MakeWizArgs {
    pub fn subcommands_provided(&self) -> bool {
        self.command.is_some()
    }

    pub fn flags_provided(&self) -> bool {
        self.executable.is_some() || self.compiler.is_some()
    }

    pub fn parse_flags(&self) -> (String, String) {
        let mut lflags = String::from("");
        let mut ldlibs = String::from("");

        //LFLAGS
        if self.thread { lflags.push_str("-lpthread "); }
        if self.math { lflags.push_str("-lm"); }

        //LDLIBS
        if self.cunit { ldlibs.push_str("-lcunit "); }
        if self.cppunit { ldlibs.push_str("-lcppunit "); }
        if self.crypto { ldlibs.push_str("-lcrypto"); }

        (lflags.trim_end().to_string(), ldlibs.trim_end().to_string())
    }
}