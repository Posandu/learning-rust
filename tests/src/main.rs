anslate code into


Ask Copilot
fn main() {
    use std::env;
    use std::fs::{self, DirEntry};
    use std::path::Path;

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    fn is_file(entry: &DirEntry) -> bool {
        entry.file_type().is_file()
    }

    fn is_dir(entry: &DirEntry) -> bool {
        entry.file_type().is_dir()
    }

    fn is_hidden_file(entry: &DirEntry) -> bool {
        is_hidden(entry) && is_file(entry)
    }

    fn is_hidden_dir(entry: &DirEntry) -> bool {
        is_hidden(entry) && is_dir(entry)
    }

    fn is_hidden_dir_with_files(entry: &DirEntry) -> bool {
        is_hidden_dir(entry) && !is_hidden_file(entry)
    }

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if is_hidden_dir_with_files(&path) {
            println!("{}", path.display());
        }
    }
}