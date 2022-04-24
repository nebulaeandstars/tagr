use clap::{Parser, Subcommand};

use crate::file::File;
use crate::tag::Tag;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Tag a set of files
    Tag {
        /// Tag to add the files to
        tag: Tag,

        /// List of files to tag
        files: Vec<File>,
    },

    /// Untag a set of files
    Untag {
        /// Tag to add the files to
        tag: Tag,

        /// List of files to untag
        files: Vec<File>,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[macro_export]
macro_rules! crash {
    ($str:literal) => {{
        eprintln!($str);
        std::process::exit(1);
    }};

    ($fmt_str:literal, $($args:expr),*) => {{
        eprintln!($fmt_str, $($args),*);
        std::process::exit(1);
    }};
}
