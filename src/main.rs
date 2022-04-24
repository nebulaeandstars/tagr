mod cli;
mod file;
mod tag;

use cli::Command;

fn main() {
    let cli = cli::parse();

    match cli.command {
        Command::Add { tag, files } => tag.add(files),
        Command::Rm { tag, files } => tag.remove(files),
        Command::Ls { tag } => tag.list_members(),
    }
}
