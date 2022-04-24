mod cli;
mod file;
mod tag;

use cli::Command;
use file::File;
use tag::Tag;

fn tag_files(tag: &Tag, files: &[File]) {
    for file in files {
        println!("adding to {}: {}", tag.name, file);
    }
}

fn untag_files(tag: &Tag, files: &[File]) {
    for file in files {
        println!("removing from {}: {}", tag.name, file);
    }
}

fn list_tagged_files(tag: &Tag) {
    for file in tag.get_members() {
        println!("{}", file);
    }
}

fn main() {
    let cli = cli::parse();

    match &cli.command {
        Command::Tag { tag, files } => tag_files(tag, files),
        Command::Untag { tag, files } => untag_files(tag, files),
        Command::Ls { tag } => list_tagged_files(tag),
    }
}
