// TODO: add proper error handling?
fn list_dir(path: &std::path::Path, nesting: u32) {
    let entries = std::fs::read_dir(path).unwrap();
    let mut padding = "".to_owned();
    for _ in 0..nesting {
        padding.push(' ');
        padding.push(' ');
    }
    for entry in entries {
        let entry = entry.unwrap();
        let entry_type = entry.file_type().unwrap();
        let entry_path = entry.path();
        let name_of_path = entry_path.as_path().file_name().unwrap().to_str().unwrap();
        if name_of_path.chars().nth(0).unwrap() == '.' {
            continue;
        }
        if entry_type.is_dir() {
            println!("{}{}", padding, name_of_path);
            list_dir(entry.path().as_path(), nesting + 1);
        } else {
            println!("{}{}", padding, name_of_path);
        }
    }
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    let path = dir.as_path();
    list_dir(path, 0);
}
