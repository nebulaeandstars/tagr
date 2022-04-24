mod cli;
mod file;
mod tag;

use cli::Command;
use file::File;
use tag::Tag;

fn tag_files(tag: Tag, files: Vec<File>) {
    for file in files {
        println!("adding to {}: {}", tag.name, file);
        tag.add(file);
    }
}

fn untag_files(tag: Tag, files: Vec<File>) {
    for file in files {
        println!("removing from {}: {}", tag.name, file);
        tag.remove(file);
    }
}

fn list_tagged_files(tag: Tag) {
    let members = tag.get_members();

    if members.len() > 0 {
        for file in tag.get_members() {
            println!("{}", file);
        }
    }
    else {
        crash!("no such tag: '{}'", tag.name)
    }
}

fn create_tagfile_dir() {
    let mut path = dirs::data_dir().unwrap();
    path.push("tagr");
    std::fs::create_dir_all(path).unwrap();
}

fn main() {
    let cli = cli::parse();

    create_tagfile_dir();

    match cli.command {
        Command::Tag { tag, files } => tag_files(tag, files),
        Command::Untag { tag, files } => untag_files(tag, files),
        Command::Ls { tag } => list_tagged_files(tag),
    }
}
