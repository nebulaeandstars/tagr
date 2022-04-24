mod cli;

use std::fs;
use std::path::PathBuf;

use cli::Command;

fn validate_files<'a>(files: &'a [PathBuf]) -> Vec<&'a PathBuf> {
    files
        .iter()
        .inspect(|file| {
            if !file.exists() {
                crash!("{:?} does not exist!", file)
            }
        })
        .collect()
}

fn tag_files(tag: &str, files: &[PathBuf]) {
    let files = validate_files(files);

    for file in files {
        println!("adding {:?} to '{}'", file, tag);
    }
}

fn untag_files(tag: &str, files: &[PathBuf]) {
    let files = validate_files(files);

    for file in files {
        println!("removing {:?} from '{}'", file, tag);
    }
}

fn main() {
    let cli = cli::parse();

    match &cli.command {
        Some(Command::Tag { tag, files }) => tag_files(tag, files),
        Some(Command::Untag { tag, files }) => untag_files(tag, files),
        None => {},
    }
}
