use std::{fs, path::Path};

// Create a label for the file/dir
pub fn label_fom_path(file: &Path) -> String {
    let file_name = file.file_name().unwrap().to_string_lossy();

    // If the file is a symlink
    if file.is_symlink() {
        let link = fs::read_link(file).expect("Failed to read symlink");
        (file_name + " -> " + link.to_string_lossy()).to_string()
    } else {
        file_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, os};
    use temp_dir::TempDir;

    #[test]
    fn test_label_from_path() {
        // Create test files
        let tmp = TempDir::new().unwrap();
        let _ = fs::write(tmp.child("document.md"), "");

        #[cfg(unix)]
        let _ = os::unix::fs::symlink(tmp.child("document.md"), tmp.child("document_symlink.md"));

        #[cfg(windows)]
        let _ = os::windows::fs::symlink_dir(
            tmp.child("document.md"),
            tmp.child("document_symlink.md"),
        );

        assert_eq!(
            "document_symlink.md -> ".to_owned() + tmp.path().to_str().unwrap() + "/document.md",
            label_fom_path(&tmp.child("document_symlink.md"))
        );
        assert_eq!("document.md", label_fom_path(&tmp.child("document.md")));
    }
}
