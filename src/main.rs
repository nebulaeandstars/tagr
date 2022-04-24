mod cli;

use std::path::PathBuf;

use cli::Command;

fn tag_files(tag: &str, files: &[PathBuf]) {
    println!("adding {:?} to '{}'", files, tag);
}

fn untag_files(tag: &str, files: &[PathBuf]) {
    println!("removing {:?} from '{}'", files, tag);
}

fn main() {
    let cli = cli::parse();

    match &cli.command {
        Some(Command::Tag { tag, files }) => tag_files(tag, files),
        Some(Command::Untag { tag, files }) => untag_files(tag, files),
        None => {},
    }
}
