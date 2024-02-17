use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod colors;
mod icons;

fn main() {
    for entry in fs::read_dir(get_dir()).unwrap() {
        // Just print the icon and the label
        println!(
            "{} {}",
            icons::icon_from_file(&entry.as_ref().unwrap().path()),
            colors::colorize_file(&entry.unwrap().path())
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
