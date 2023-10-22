// This file contains a crud load of functions that are used throughout the program.
// 
//To Read, Write and Create files
use std::fs::{File, OpenOptions, self};
use std::io::{self, Read, Write};
use std::path::Path;


// ? Files ---------------------------------------------------------------------------------------------------------------------------------------------------------


// * CREATE
/// Creates a file with the given content.
///
/// # Arguments
///
/// - `path` - A string slice representing the path where the file should be created.
/// - `filename` - A string slice representing the name of the file.
/// - `content` - A string slice containing the content to write to the file.
///
/// # Returns
///
/// - A `Result` where:
///   - `Ok(())` indicates success in creating the file and writing the content.
///   - `Err(io::Error)` contains an error if the file cannot be created or written.
///
/// # Examples
/// ```rust
/// use util::files::create_file;
/// 
/// let path = ".";
/// let filename = "example.txt";
/// let content = "Hello, Rust!";
/// let result = create_file(path, filename, content);
/// assert!(result.is_ok());
/// ```
fn create_file(path: &str, filename: &str, content: &str) -> Result<(), io::Error> {
    let file_path = Path::new(path).join(filename);

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}


// * READ
/// Reads a file given its path and filename.
///
/// # Arguments
///
/// - `path` - A string slice representing the path to the file.
/// - `filename` - A string slice representing the name of the file.
///
/// # Returns
///
/// - A `Result` where:
///   - `Ok(String)` contains the content of the file as a string if successful.
///   - `Err(io::Error)` contains an error if the file cannot be opened or read.
///
/// # Example
///
/// ```
/// use std::fs::write;
/// let path = "example.txt";
/// let content = "Hello, Rust!";
/// write(path, content).expect("Unable to write file.");
/// let result = read_file(path, "example.txt");
/// assert_eq!(result, Ok("Hello, Rust!".to_string()));
/// ```
fn read_file(path: &str, filename: &str) -> Result<String, io::Error> {
    let file_path = Path::new(path).join(filename);

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}


// * UPDATE
/// Updates an existing file with new content.
///
/// If the file does not exist, it will be created.
///
/// # Arguments
///
/// - `path` - A string slice representing the path where the file should be updated or created.
/// - `filename` - A string slice representing the name of the file.
/// - `content` - A string slice containing the new content to write to the file.
///
/// # Returns
///
/// - A `Result` where:
///   - `Ok(())` indicates success in updating or creating the file with the new content.
///   - `Err(io::Error)` contains an error if the file cannot be updated or created.
///
/// # Example
///
/// ```
/// let path = ".";
/// let filename = "example.txt";
/// let content = "Updated content!";
/// let result = update_file(path, filename, content);
/// assert!(result.is_ok());
/// ```
fn update_file(path: &str, filename: &str, content: &str) -> Result<(), io::Error> {
    let file_path = Path::new(path).join(filename);

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}


// * DELETE
/// Deletes a file given its path and filename.
///
/// # Arguments
///
/// - `path` - A string slice representing the path where the file is located.
/// - `filename` - A string slice representing the name of the file to be deleted.
///
/// # Returns
///
/// - A `Result` where:
///   - `Ok(())` indicates success in deleting the file.
///   - `Err(io::Error)` contains an error if the file cannot be deleted.
///
/// # Example
///
/// ```
/// let path = ".";
/// let filename = "example.txt";
/// let result = delete_file(path, filename);
/// assert!(result.is_ok());
/// ```
fn delete_file(path: &str, filename: &str) -> Result<(), io::Error> {
    let file_path = format!("{}/{}", path, filename);

    match fs::remove_file(&file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

