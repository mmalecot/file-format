/*!
This is a program that provides a simplified version of the UNIX `file` command. It determines the
file format of a given file and prints out the result.

The program first parses the command-line arguments passed to it and finds the maximum width of the
input file paths. This is used to align the output when printing the results.

Next, the program iterates over the input file paths passed as command-line arguments. For each file
path, it checks whether it is a symbolic link or a directory. If so, the program prints out a
special message. Otherwise, the program determines the file format using the `FileFormat::from_file`
function and prints out the results.
*/

use file_format::FileFormat;
use std::{env, io::Result, ops::Add, path::Path};

fn main() -> Result<()> {
    let width = env::args()
        .skip(1)
        .map(|input| input.chars().count())
        .max_by_key(|&count| count)
        .unwrap_or_default()
        .add(1);
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let input = format!("{arg}:");
        if path.is_symlink() {
            println!("{input:width$} Symbolic Link");
        } else if path.is_dir() {
            println!("{input:width$} Directory");
        } else {
            let fmt = FileFormat::from_file(path)?;
            let name = fmt.name();
            if let Some(short_name) = fmt.short_name() {
                println!("{input:width$} {name} ({short_name})");
            } else {
                println!("{input:width$} {name}");
            }
        }
    }
    Ok(())
}
