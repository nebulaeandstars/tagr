use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Tag a set of files
    Tag {
        /// Tag to add the files to
        tag: String,

        /// List of files to tag
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },

    /// Untag a set of files
    Untag {
        /// Tag to add the files to
        tag: String,

        /// List of files to untag
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
