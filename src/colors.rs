use colored::{ColoredString, Colorize};
use std::path::Path;

// Apply color to a label with information from a std::path::Path
pub fn colorize_label(label: &String, file: &Path) -> ColoredString {
    // If the file is a symlink
    if file.is_symlink() {
        label.yellow()
    } else {
        // If the file is a directory
        if file.is_dir() {
            label.blue()
        } else {
            label.white()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use temp_dir::TempDir;

    #[test]
    fn test_colorize_label() {
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
            colorize_label(&"document.md".to_string(), &tmp.child("document.md")),
            "document.md".white()
        );
        assert_eq!(
            colorize_label(&"image.png".to_string(), &tmp.child("image.png")),
            "image.png".white()
        );
        assert_eq!(
            colorize_label(&".gitignore".to_string(), &tmp.child(".gitignore")),
            ".gitignore".white()
        );
        assert_eq!(
            colorize_label(&"config".to_string(), &tmp.child("config")),
            "config".blue()
        );
        assert_eq!(
            colorize_label(
                &"config_symlink -> config".to_string(),
                &tmp.child("config_symlink")
            ),
            "config_symlink -> config".yellow()
        );
    }
}
