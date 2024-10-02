//! This module provides functions for performing CRUD (Create, Read, Update, Delete) operations on files.
//! 
//! This module simplifies file operations in Rust, making it easy to manage and manipulate files in your applications.
//! 
//! It offers a simple and efficient way to work with files, allowing you to create, read, update, and delete files with ease.
//!
//!
//! # Contents
//! - [Create](fn.create_file.html) a file with the given content.
//! - [Read](fn.read_file.html) a file given its path and filename.
//! - [Update](fn.update_file.html) an existing file with new content.
//! - [Delete](fn.delete_file.html) a file given its path and filename.
//! 
//! # Examples
//! In this example we create a file, then read it and update it.
//! 
//! ```rust
//! use dev_utils::files::crud::*;
//! 
//! let path = "test/";  // Specify the path where the file should be created.
//! let filename = "example.txt";  // Also specify the file format & path.
//! let content = "Hello, Rust!";  // Specify the content to write to the file.
//! 
//! let result = create_file(path, filename, content);  // Create the file.
//! assert!(result.is_ok());  // Check if the file was created successfully.
//! 
//! let result = read_file(path, filename);  // Read the file.
//! assert_eq!(result.unwrap(), "Hello, Rust!");  // Check if the file content is correct.
//! 
//! let content = "Updated content!";  // Specify the new content to write to the file.
//! let result = update_file(path, filename, content);  // Update the file.
//! ```
// todo: FIX THE EXAMPLES (Change from Result<String, String> to Result<(), String>)
use std::fmt::format;
use std::fs::{File, OpenOptions, self};
use std::io::{self, Read, Write, Error};
use std::path::{Path, PathBuf};


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
/// use dev_utils::files::crud::create_file;
/// 
/// let path = "test/";
/// let filename = "example.txt";
/// let content = "Hello, Rust!";
/// let result = create_file(path, filename, content);
/// assert!(result.is_ok());
/// ```
// todo: Update it to now also return the file-path of the created file.
pub fn create_file(path: &str, filename: &str, content: &str) -> Result<String, io::Error> {
    let file_path = Path::new(path).join(filename);  // Get the full path to store the file.
    let mut file = File::create(&file_path)?;  // Create the file.
    file.write_all(content.as_bytes())?;  // Write the content to the file.
    Ok(format!("Successfully created file: {file_path:?}"))
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
/// ```
/// use std::fs::write;
/// use dev_utils::files::crud::read_file;
/// 
/// let path = "test/";
/// 
/// // Create a file to read from.
/// let file_name = "example.txt";  // Also specify the file format & path.
/// let content = "Hello, Rust!";
/// write(format!("{}{}", path ,file_name), content).expect("Unable to write file.");  // Write file to the current directory.
/// 
/// // Read the file.
/// let result = read_file(path, "example.txt");
/// assert!(result.is_ok());  // Check if the file was read successfully.
/// assert_eq!(result.unwrap(), Ok(()));  // Check if the file content is correct.
/// ```
pub fn read_file(path: &str, filename: &str) -> Result<String, io::Error> {
    let file_path = Path::new(path).join(filename);
    let mut file = File::open(&file_path)?;  // Open the file.

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        // if Ok, return the content of the file
        Ok(_) => Ok(content),
        Err(e) => Err(Error::new(io::ErrorKind::AddrNotAvailable, format!("Error reading file: {}", e))),
    }
}


// * UPDATE
/// Updates an existing file with new content.
///
/// If the file does not exist, it will be created. If it does exist, it will be overwritten with the new content.
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
/// ```
/// use dev_utils::files::crud::update_file;
/// 
/// let path = "test/";
/// let filename = "example.txt";
/// let content = "Updated content!";
/// let result = update_file(path, filename, content);
/// assert!(result.is_ok());
/// ```
pub fn update_file(path: &str, filename: &str, content: &str) -> Result<String, io::Error> {
    let file_path = Path::new(path).join(filename);
    let mut file = match OpenOptions::new()
        .write(true)  // Open the file in write mode.
        .create(true)  // Create the file if it does not exist.
        .truncate(true)  // Truncate the file to 0 bytes. (Meaning it will be overwritten)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(d) => Ok(format!("Successfully updated file: {}", file_path.display())),
        Err(e) => Err(Error::new(io::ErrorKind::AddrNotAvailable, format!("Error writing to file: {}", e))),
    }
}


// * APPEND (Add to file)
/// Appends content to an existing file.
///
/// # Arguments
///
/// - `path` - A string slice representing the path where the file is located.
/// - `filename` - A string slice representing the name of the file to be updated.
/// - `content` - A string slice representing the content to append to the file.
///
/// # Returns
///
/// - A `Result` where:
///   - `Ok(())` indicates success in appending to the file.
///   - `Err(io::Error)` contains an error if the file cannot be updated.
///
/// # Example
///
/// ```
/// use dev_utils::files::crud::create_file;
/// use dev_utils::files::crud::add_to_file;
/// 
/// let path = "test/";
/// let filename = "example.txt";
/// let content = "Hello, Rust!";
/// // Create a file to append to.
/// create_file(path, filename, content).expect("Unable to create file.");
/// 
/// // Append to the file.
/// let append_content = " Appended content!";
/// let result = add_to_file(path, filename, append_content);
/// assert!(result.is_ok());
/// ```
pub fn append_file(path: &str, filename: &str, content: &str) -> Result<String, io::Error> {
    let file_path = Path::new(path).join(filename);

    let mut file = match OpenOptions::new()
        .write(true)  // Open the file in write mode.
        .append(true) // Set the file to append mode.
        .open(&file_path)
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(format!("Successfully appended to file: {}", file_path.display())),
        Err(e) => Err(Error::new(io::ErrorKind::AddrNotAvailable, format!("Error writing to file: {}", e))),
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
/// use dev_utils::files::crud::create_file;
/// use dev_utils::files::crud::delete_file;
/// 
/// let path = "test/";
/// let filename = "example.txt";
/// let content = "Hello, Rust!";
/// // Create a file to delete.
/// create_file(path, filename, content).expect("Unable to create file.");
/// 
/// // Delete the file.
/// let result = delete_file(path, filename);
/// assert!(result.is_ok());
/// ```
pub fn delete_file(path: &str, filename: &str) -> Result<String, io::Error> {
    let file_path = format!("{}/{}", path, filename);

    match fs::remove_file(&file_path) {
        Ok(_) => Ok(format!("Successfully deleted file: {}", file_path)),
        Err(e) => Err(Error::new(io::ErrorKind::InvalidInput, format!("Error deleting file: {}", e))),
    }
}
