use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "now",
    version,
    about = "Invisible, reproducible local toolchains"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a command inside an ephemeral project-aware environment.
    Run {
        /// Force a specific container image instead of inferring one.
        #[arg(long)]
        image: Option<String>,

        /// Command and arguments to execute.
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,
    },

    /// Start an interactive shell for a toolchain.
    Shell {
        /// Toolchain selector, such as python@3.9 or node@20.
        toolchain: String,
    },

    /// Generate a starter .now.yaml.
    Init,
}
