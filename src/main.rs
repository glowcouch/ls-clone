use clap::{arg, command};
use std::{env, fs, path::PathBuf};

static INDENT_WIDTH: i32 = 2;

mod colors;
mod icons;
mod labels;

fn main() {
    from_arguments();
}

// Run the program from the command line arguments
fn from_arguments() {
    // Set up clap arguments
    let matches = command!()
        .arg(arg!([path] "Optional path (defaults to current directory)"))
        .arg(arg!(-a --all "Don't ignore entries starting with ."))
        .arg(arg!(-r --recursive <depth> "Recursively lists files"))
        .get_matches();

    // Should we hide dots
    let hide_dots: bool = *matches.get_one::<bool>("all").unwrap();

    let dir: PathBuf;

    // Run in current dir or dir from arguments
    if let Some(path) = matches.get_one::<String>("path") {
        dir = PathBuf::from(path.clone());
    } else {
        dir = env::current_dir().unwrap();
    }

    // Recursive or not
    if let Some(recursive) = matches.get_one::<String>("recursive") {
        let depth = recursive.parse::<i32>();
        list_dir(&dir, 0, true, depth.clone().unwrap(), hide_dots);
    } else {
        list_dir(&dir, 0, false, 0, hide_dots);
    }
}

// List the files in a directory
fn list_dir(dir: &PathBuf, indentation: i32, recursive: bool, depth: i32, hide_dots: bool) {
    for entry in fs::read_dir(dir).unwrap() {
        // Check if file starts with .
        if (!entry
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(".")
            && !hide_dots)
            || hide_dots
        {
            // Indentation
            for _ in 0..indentation {
                print!(" ")
            }

            // Print icon and label
            println!(
                "{} {}",
                icons::icon_from_file(&entry.as_ref().unwrap().path()),
                colors::colorize_label(
                    &labels::label_fom_path(&entry.as_ref().unwrap().path()),
                    &entry.as_ref().unwrap().path()
                )
            );

            // Recurse if applicable
            if entry.as_ref().unwrap().path().is_dir() && recursive && (depth > 0 || depth < 0) {
                list_dir(
                    &entry.unwrap().path(),
                    indentation + INDENT_WIDTH,
                    true,
                    depth - 1,
                    hide_dots,
                );
            }
        }
    }
}
