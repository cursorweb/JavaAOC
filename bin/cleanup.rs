//! Removes data.txt files
//! You don't need them all
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const IGNORE: [&'static str; 2] = [".\\target", ".\\.git"];

fn _list_files(files: &mut Vec<PathBuf>, path: &Path) {
    if fs::metadata(&path).unwrap().is_dir() {
        let paths = fs::read_dir(&path).unwrap();
        for path_result in paths {
            let path = path_result.unwrap().path();

            if IGNORE.contains(&path.to_str().unwrap()) {
                continue;
            }

            if fs::metadata(&path).unwrap().is_dir() {
                _list_files(files, &path);
            } else {
                files.push(path);
            }
        }
    }
}

fn list_files(path: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    _list_files(&mut out, &path);
    out
}

fn main() -> io::Result<()> {
    let paths = list_files(&Path::new("."));

    for path in paths {
        if path.extension().map_or(false, |e| e == "txt") && path.ends_with("data") {
            println!("Deleting {path:?}");
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
