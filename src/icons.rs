use colored::{ColoredString, Colorize};
use nerd_font_symbols::md;
use std::{fs, path::Path};

// Get a nerd font icon from a std::path::Path file
pub fn icon_from_file(file: &Path) -> ColoredString {
    if file.is_dir() {
        // Empty folder
        if let Ok(read_dir) = fs::read_dir(file) {
            if read_dir.peekable().peek().is_some() {
                md::MD_FOLDER.blue()
            } else {
                md::MD_FOLDER_OUTLINE.blue()
            }
        } else {
            md::MD_FOLDER_QUESTION.blue()
        }
    } else {
        if let Some(ext) = file.extension() {
            // File extension icons
            match ext.to_str().unwrap() {
                "rs" => md::MD_LANGUAGE_RUST.red(), // rust files
                "toml" => md::MD_COG.blue(),        // Toml files
                "conf" => md::MD_COG.black(),
                "lock" => md::MD_LOCK.red(),
                "nix" => md::MD_NIX.blue(), // Nix language files
                "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "svg" | "apng" | "kra"
                | "ico" | "tiff" => md::MD_IMAGE.magenta(), // Images
                "md" => md::MD_LANGUAGE_MARKDOWN.blue(),
                "docx" | "pdf" | "txt" => md::MD_FILE_DOCUMENT.blue(),
                "zip" | "rar" => md::MD_ZIP_BOX.blue(), // Compressed files
                "c" => md::MD_LANGUAGE_C.blue(),
                "cpp" => md::MD_LANGUAGE_CPP.blue(),
                "jar" | "java" => md::MD_LANGUAGE_JAVA.blue(),
                "cs" => md::MD_LANGUAGE_CSHARP.blue(),
                "css" => md::MD_LANGUAGE_CSS3.blue(),
                "js" => md::MD_LANGUAGE_JAVASCRIPT.blue(),
                "lua" => md::MD_LANGUAGE_LUA.blue(),
                "php" => md::MD_LANGUAGE_PHP.blue(),
                "py" => md::MD_LANGUAGE_PYTHON.blue(),
                "go" => md::MD_LANGUAGE_GO.blue(),
                "html" => md::MD_LANGUAGE_HTML5.blue(),
                "sh" => nerd_font_symbols::dev::DEV_TERMINAL.blue(),
                "mp4" | "mov" | "avi" | "wmv" | "flv" | "f4v" | "mkv" | "webm" | "avchd"
                | "m4v" => md::MD_VIDEO.blue(), // Videos
                "sock" => md::MD_POWER_PLUG.blue(),
                _ => " ".white(),
            }
        } else {
            // Exact file name matches
            match file.file_name().unwrap().to_str().unwrap() {
                ".gitignore" => md::MD_GIT.black(), // Gitignores
                _ => md::MD_TEXT_BOX.white(),       // Catch all for unknown files
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_font_symbols::md;
    use std::fs;
    use temp_dir::TempDir;

    #[test]
    fn test_icon_from_file() {
        // Create test files
        let tmp = TempDir::new().unwrap();
        let _ = fs::write(tmp.child("image.png"), "");
        let _ = fs::write(tmp.child("config.toml"), "");

        assert_eq!(
            icon_from_file(&tmp.child("image.png")),
            md::MD_IMAGE.magenta()
        );
        assert_eq!(icon_from_file(&tmp.child("config.toml")), md::MD_COG.blue());
    }
}
