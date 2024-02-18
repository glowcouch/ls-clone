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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use temp_dir::TempDir;

    #[test]
    fn test_colorize_file() {
        // Create test files
        let tmp = TempDir::new().unwrap();
        let _ = fs::write(tmp.child("image.png"), "");
        let _ = fs::write(tmp.child("document.md"), "");
        let _ = fs::write(tmp.child(".gitignore"), "");
        let _ = fs::create_dir(tmp.child("config"));

        #[cfg(unix)]
        let _ = std::os::unix::fs::symlink(tmp.child("config"), tmp.child("config_symlink"));

        #[cfg(windows)]
        let _ = std::os::windows::fs::symlink_dir(tmp.child("config_symlink"), tmp.child("config"));

        assert_eq!(
            colorize_file(&tmp.child("document.md")),
            "document.md".white()
        );
        assert_eq!(colorize_file(&tmp.child("image.png")), "image.png".white());
        assert_eq!(
            colorize_file(&tmp.child(".gitignore")),
            ".gitignore".white()
        );
        assert_eq!(colorize_file(&tmp.child("config")), "config".blue());
        assert_eq!(
            colorize_file(&tmp.child("config_symlink")),
            "config_symlink".yellow()
        );
    }
}
