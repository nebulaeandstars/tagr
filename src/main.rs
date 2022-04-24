use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    verbosity: usize,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Tag { files, tag }) =>
            println!("adding {:?} to '{}'", files, tag),
        Some(Command::Untag { files, tag }) =>
            println!("removing {:?} from '{}'", files, tag),
        None => {},
    }
}
