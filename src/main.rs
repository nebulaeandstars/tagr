mod cli;
mod tag;

use std::path::PathBuf;

use cli::Command;
use tag::Tag;

fn validate_files(files: &[PathBuf]) -> Vec<&PathBuf> {
    files
        .iter()
        .inspect(|file| {
            if !file.exists() {
                crash!("{:?} does not exist!", file)
            }
        })
        .collect()
}

fn tag_files(tag: &Tag, files: &[PathBuf]) {
    let files = validate_files(files);

    println!("{:?}", tag.tagfile);

    for file in files {
        println!("adding to {}: {:?}", tag.name, file);
    }
}

fn untag_files(tag: &Tag, files: &[PathBuf]) {
    let files = validate_files(files);

    for file in files {
        println!("removing from {}: {:?}", tag.name, file);
    }
}

fn main() {
    let cli = cli::parse();

    match &cli.command {
        Command::Tag { tag, files } => tag_files(tag, files),
        Command::Untag { tag, files } => untag_files(tag, files),
    }
}
