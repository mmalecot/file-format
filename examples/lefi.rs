use clap::Parser;
use colored::*;
use file_format::FileFormat;
use std::{io::Result, path::Path};

#[derive(Debug, Parser)]
#[clap(about = "Simplified version of the UNIX file command.")]
struct Args {
    #[clap(value_parser)]
    input: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let width = args
        .input
        .iter()
        .map(|input| input.len())
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
                name = match format.media_type().split('/').next().unwrap_or_default() {
                    "audio" => format.name().cyan(),
                    "font" => format.name().yellow(),
                    "image" => format.name().magenta(),
                    "model" => format.name().blue(),
                    "video" => format.name().green(),
                    _ => format.name().white(),
                },
                extension = format.extension(),
                media_type = format.media_type().underline(),
            );
        }
    }
    Ok(())
}
