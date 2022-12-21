//! Simplified version of the UNIX file command.

use file_format::{FileFormat, Kind};
use std::{env, fmt::Display, io::Result, path::Path};

trait Decorate: Display {
    fn underline(&self) -> String {
        format!("\x1B[4m{self}\x1B[0m")
    }

    fn red(&self) -> String {
        format!("\x1B[31m{self}\x1B[0m")
    }

    fn green(&self) -> String {
        format!("\x1B[32m{self}\x1B[0m")
    }

    fn yellow(&self) -> String {
        format!("\x1B[33m{self}\x1B[0m")
    }

    fn blue(&self) -> String {
        format!("\x1B[34m{self}\x1B[0m")
    }

    fn magenta(&self) -> String {
        format!("\x1B[35m{self}\x1B[0m")
    }

    fn cyan(&self) -> String {
        format!("\x1B[36m{self}\x1B[0m")
    }
}

impl<D: Display> Decorate for D {}

fn main() -> Result<()> {
    let width = env::args()
        .skip(1)
        .map(|input| input.chars().count())
        .collect::<Vec<usize>>()
        .into_iter()
        .max()
        .unwrap_or_default();
    for input in env::args().skip(1) {
        let path = Path::new(&input);
        if path.is_symlink() {
            println!(
                "{input:width$} {name} • {media_type}",
                name = "Symbolic link".red(),
                media_type = "inode/symlink".underline(),
            );
        } else if path.is_dir() {
            println!(
                "{input:width$} {name} • {media_type}",
                name = "Directory".red(),
                media_type = "inode/directory".underline(),
            );
        } else {
            let format = FileFormat::from_file(path)?;
            println!(
                "{input:width$} {name} • {extension} • {media_type}",
                name = match format.kind() {
                    Kind::Application => format.blue(),
                    Kind::Audio => format.cyan(),
                    Kind::Image => format.magenta(),
                    Kind::Text => format.yellow(),
                    Kind::Video => format.green(),
                    _ => format.to_string(),
                },
                extension = format.extension(),
                media_type = format.media_type().underline(),
            );
        }
    }
    Ok(())
}
