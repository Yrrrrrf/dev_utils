use std::path::{Path, PathBuf}; 
use std::io::BufRead;



pub fn find_cargo_toml_files(start_dir: &Path) -> Vec<PathBuf> {
    let mut cargo_toml_files = Vec::new();

    if start_dir.is_dir() {
        for entry in std::fs::read_dir(start_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
                cargo_toml_files.push(path.clone());
            } else if path.is_dir() {
                cargo_toml_files.extend(find_cargo_toml_files(&path));
            }
        }
    }

    cargo_toml_files
}

pub fn read_cargo_toml_file(cargo_toml_path: &Path) {
    let mut name = String::new();
    let mut version = String::new();
    let mut authors = String::new();

    if let Ok(file) = std::fs::File::open(cargo_toml_path) {
        let mut lines = std::io::BufReader::new(file).lines().peekable();

        while let Some(Ok(line)) = lines.next() {
            if line.starts_with("[package]") {
                while let Some(Ok(line)) = lines.next() {
                    match line {
                        _ if line.starts_with("name =") => {
                            name = line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase()
                        }
                        _ if line.starts_with("version =") => {
                            version = format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", ""))
                        }
                        _ if line.starts_with("authors =") => {
                            authors = line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", "")
                        }
                        _ if line.starts_with("[") => break,
                        _ => (),
                    }
                }
                println!("{} {}\nAuthors: {}\n", name, version, authors);
                break;
            }
        }
    }
}

