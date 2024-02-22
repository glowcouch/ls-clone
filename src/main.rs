use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod colors;
mod icons;
mod labels;

fn main() {
    list_dir(&get_dir(), 0);
}

// List the files in a directory
fn list_dir(dir: &PathBuf, indentation: i32) {
    for entry in fs::read_dir(dir).unwrap() {
        for _ in 0..indentation {
            print!(" ")
        }
        println!(
            "{} {}",
            icons::icon_from_file(&entry.as_ref().unwrap().path()),
            colors::colorize_label(
                &labels::label_fom_path(&entry.as_ref().unwrap().path()),
                &entry.unwrap().path()
            )
        );
    }
}

// Get the directory the program should run on
fn get_dir() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if Path::new(&args[1]).exists() {
            PathBuf::from(args[1].clone())
        } else {
            "".to_string().into()
        }
    } else {
        env::current_dir().unwrap()
    }
}
