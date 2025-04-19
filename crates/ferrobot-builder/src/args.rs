use core::fmt;

#[derive(clap::Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub operation: Operation,
}

#[derive(clap::Subcommand)]
pub enum Operation {
    /// Regenerate FFI Bindings
    Regenerate,

    /// Build the Robot Code
    Build {
        /// Debug or Release
        #[arg(value_enum, default_value_t = BuildMode::Debug)]
        mode: BuildMode,
    },

    /// Simulate the Robot Code
    Simulate {
        /// Debug or Release
        #[arg(value_enum, default_value_t = BuildMode::Debug)]
        mode: BuildMode,
    },

    /// Deploy the Robot Code
    Deploy {
        /// Debug or Release
        #[arg(value_enum, default_value_t = BuildMode::Debug)]
        mode: BuildMode,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum BuildMode {
    Debug,
    Release,
}

impl fmt::Display for BuildMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildMode::Debug => write!(f, "debug"),
            BuildMode::Release => write!(f, "release"),
        }
    }
}
