mod cli;
mod file;
mod tag;

use cli::Command;
use file::File;
use tag::Tag;

fn tag_files(tag: Tag, files: Vec<File>) {
    for file in files {
        tag.add(file);
    }
}

fn untag_files(tag: Tag, files: Vec<File>) {
    for file in files {
        tag.remove(file);
    }
}

fn list_tagged_files(tag: Tag) {
    let members = tag.get_members();

    if members.is_empty() {
        crash!("no such tag: '{}'", tag.name)
    }
    else {
        for file in tag.get_members() {
            println!("{}", file);
        }
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
        Command::Add { tag, files } => tag_files(tag, files),
        Command::Rm { tag, files } => untag_files(tag, files),
        Command::Ls { tag } => list_tagged_files(tag),
    }
}
