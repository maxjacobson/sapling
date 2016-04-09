use std::env::current_dir;
use std::fs::read_dir;
use std::path::Path;

// TODO: add proper error handling?
fn list_dir(path: &Path, nesting: u32) {
    let entries = read_dir(path).unwrap();
    let mut padding = "".to_owned();
    for _ in 0..nesting {
        padding.push('\t');
    }
    for entry in entries {
        let entry = entry.unwrap();
        let entry_type = entry.file_type().unwrap();
        let entry_path = entry.path();
        let name_of_path = entry_path.as_path().to_str().unwrap();
        if entry_type.is_dir() {
            println!("{} Dir: {}", padding, name_of_path);
            list_dir(entry.path().as_path(), nesting + 1);
        } else {
            println!("{} File: {}", padding, name_of_path);
        }
    }
}

fn main() {
    let dir = current_dir().unwrap();
    let path = dir.as_path();
    list_dir(path, 0);
}
