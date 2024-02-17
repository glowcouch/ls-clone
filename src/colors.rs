use colored::{ColoredString, Colorize};
use std::path::Path;

// Apply color to a file name from a std::path::Path
pub fn colorize_file(file: &Path) -> ColoredString {
    let file_name: &str = file.file_name().unwrap().to_str().unwrap();

    // If the file is a symlink
    if file.is_symlink() {
        file_name.yellow()
    } else {
        // If the file is a directory
        if file.is_dir() {
            file_name.blue()
        } else {
            file_name.white()
        }
    }
}
