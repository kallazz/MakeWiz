use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about)]
pub struct MakeWizArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set the compiler name for THIS Makefile
    #[arg(short, long, value_name = "COMPILER_NAME")]
    pub compiler: Option<String>,

    /// Set the executable name for THIS Makefile
    #[arg(short, long, value_name = "EXECUTABLE_NAME")]
    pub executable: Option<String>,
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
}