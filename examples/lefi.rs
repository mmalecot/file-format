use clap::Parser;
use colored::*;
use file_format::{FileFormat, Kind};
use std::{io::Result, path::Path};

#[derive(Debug, Parser)]
#[clap(about = "Simplified version of the UNIX file command.")]
struct Args {
    input: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let width = args
        .input
        .iter()
        .map(|input| input.chars().count())
        .collect::<Vec<usize>>()
        .into_iter()
        .max()
        .unwrap_or_default();
    for input in args.input {
        let path = Path::new(&input);
        if path.is_symlink() {
            println!(
                "{input:width$} {name} - {media_type}",
                name = "Symbolic link".red(),
                media_type = "inode/symlink".underline(),
            );
        } else if path.is_dir() {
            println!(
                "{input:width$} {name} - {media_type}",
                name = "Directory".red(),
                media_type = "inode/directory".underline(),
            );
        } else {
            let format = FileFormat::from_file(path)?;
            println!(
                "{input:width$} {name} ({extension}) - {media_type}",
                name = match format.kind() {
                    Kind::Application => format.name().blue(),
                    Kind::Audio => format.name().cyan(),
                    Kind::Image => format.name().magenta(),
                    Kind::Text => format.name().yellow(),
                    Kind::Video => format.name().green(),
                    _ => format.name().white(),
                },
                extension = format.extension(),
                media_type = format.media_type().underline(),
            );
        }
    }
    Ok(())
}
