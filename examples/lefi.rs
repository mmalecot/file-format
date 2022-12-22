//! Simplified version of the UNIX file command.

use file_format::FileFormat;
use std::{env, io::Result, path::Path};

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
            println!("{input:width$} Symbolic link • inode/symlink");
        } else if path.is_dir() {
            println!("{input:width$} Directory • inode/directory");
        } else {
            let format = FileFormat::from_file(path)?;
            println!(
                "{input:width$} {format} • {extension} • {media_type}",
                extension = format.extension(),
                media_type = format.media_type()
            );
        }
    }
    Ok(())
}
