//! This module provides advanced functions for file and directory operations.
//! 
//! It offers a simple and efficient way to work with files and directories,
//! allowing you to create, read, update, delete, list, copy, move, and rename files with ease.
//!
//! # Features
//! - CRUD operations on files
//! - Listing directory contents
//! - Copying, moving, and renaming files
//! - Error handling with custom error types
//! - All operations use only the Rust standard library
//! 
//! # Examples
//! ```
//! use dev_utils::file::*;
//! 
//! // Create a new file
//! let file_path = create("test.txt", "Hello, World!").unwrap();
//! 
//! // Read the file contents
//! let content = read(&file_path).unwrap();
//! assert_eq!(content, "Hello, World!");
//! 
//! // Update the file contents
//! update(&file_path, "Updated content").unwrap();
//! assert_eq!(read(&file_path).unwrap(), "Updated content");
//! 
//! // Append to the file
//! append(&file_path, " Appended content").unwrap();
//! assert_eq!(read(&file_path).unwrap(), "Updated content Appended content");
//! 
//! // Delete the file
//! delete(&file_path).unwrap();
//! assert!(!file_path.exists());
//! ```
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions, DirEntry};
use std::io::{self, Read, Write, Error};
use std::fmt;

/// Custom error type for file operations.
#[derive(Debug)]
pub enum FileError {
    /// Represents an IO error from the standard library.
    Io(io::Error),
    /// Represents a path-related error
    PathError(String),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::Io(err) => write!(f, "IO error: {}", err),
            FileError::PathError(err) => write!(f, "Path error: {}", err),
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {FileError::Io(err)}
}

/// Custom Result type for file operations.
type Result<T> = std::result::Result<T, FileError>;

/// Creates a new file with the given content.
///
/// If the file already exists, it will be overwritten.
///
/// # Arguments
///
/// * `path` - The path where the file should be created.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// Returns a `Result` containing the `PathBuf` of the created file, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::create;
///
/// let file_path = create("example.txt", "Hello, World!").unwrap();
/// assert!(file_path.exists());
/// ```
pub fn create<P: AsRef<Path>>(path: P, content: &str) -> Result<PathBuf> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(path.to_owned())
}

/// Reads the contents of a file.
///
/// # Arguments
///
/// * `path` - The path of the file to read.
///
/// # Returns
///
/// Returns a `Result` containing the file contents as a `String`, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, read};
///
/// let file_path = create("example.txt", "Hello, World!").unwrap();
/// let content = read(&file_path).unwrap();
/// assert_eq!(content, "Hello, World!");
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Updates the contents of a file.
///
/// If the file doesn't exist, it will be created.
///
/// # Arguments
///
/// * `path` - The path of the file to update.
/// * `content` - The new content to write to the file.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, update, read};
///
/// let file_path = create("example.txt", "Hello").unwrap();
/// update(&file_path, "Updated content").unwrap();
/// assert_eq!(read(&file_path).unwrap(), "Updated content");
/// ```
pub fn update<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Appends content to the end of a file.
///
/// If the file doesn't exist, it will be created.
///
/// # Arguments
///
/// * `path` - The path of the file to append to.
/// * `content` - The content to append to the file.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, append, read};
///
/// let file_path = create("example.txt", "Hello").unwrap();
/// append(&file_path, ", World!").unwrap();
/// assert_eq!(read(&file_path).unwrap(), "Hello, World!");
/// ```
pub fn append<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Deletes a file.
///
/// # Arguments
///
/// * `path` - The path of the file to delete.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, delete};
/// use std::path::Path;
///
/// let file_path = create("example.txt", "").unwrap();
/// delete(&file_path).unwrap();
/// assert!(!Path::new("example.txt").exists());
/// ```
pub fn delete<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

/// Lists the contents of a directory.
///
/// # Arguments
///
/// * `path` - The path of the directory to list.
///
/// # Returns
///
/// Returns a `Result` containing a `Vec<PathBuf>` of the directory contents, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, list};
/// use std::path::Path;
///
/// create("dir/file1.txt", "").unwrap();
/// create("dir/file2.txt", "").unwrap();
/// let contents = list("dir").unwrap();
/// assert_eq!(contents.len(), 2);
/// ```
pub fn list<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();
    Ok(entries)
}

/// Copies a file from one location to another.
///
/// # Arguments
///
/// * `from` - The path of the file to copy.
/// * `to` - The path where the file should be copied to.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, copy, read};
///
/// let original = create("original.txt", "Hello").unwrap();
/// copy(&original, "copy.txt").unwrap();
/// assert_eq!(read("copy.txt").unwrap(), "Hello");
/// ```
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    fs::copy(from, to)?;
    Ok(())
}

/// Moves a file from one location to another.
///
/// # Arguments
///
/// * `from` - The current path of the file.
/// * `to` - The new path for the file.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, mv, read};
/// use std::path::Path;
///
/// let original = create("original.txt", "Hello").unwrap();
/// mv(&original, "moved.txt").unwrap();
/// assert!(!Path::new("original.txt").exists());
/// assert_eq!(read("moved.txt").unwrap(), "Hello");
/// ```
pub fn mv<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}


/// Renames a file.
///
/// This function is identical to `mv`.
///
/// # Arguments
///
/// * `from` - The current path of the file.
/// * `to` - The new path for the file.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, rename, read};
/// use std::path::Path;
///
/// let original = create("original.txt", "Hello").unwrap();
/// rename(&original, "renamed.txt").unwrap();
/// assert!(!Path::new("original.txt").exists());
/// assert_eq!(read("renamed.txt").unwrap(), "Hello");
/// ```
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

// * Advanced functionality
/// Recursively copies a directory and its contents.
///
/// # Arguments
///
/// * `from` - The path of the directory to copy.
/// * `to` - The path where the directory should be copied to.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, recursive_copy, read};
/// use std::path::Path;
///
/// create("dir/file1.txt", "Hello").unwrap();
/// create("dir/subdir/file2.txt", "World").unwrap();
/// recursive_copy("dir", "copy_dir").unwrap();
/// assert!(Path::new("copy_dir/file1.txt").exists());
/// assert!(Path::new("copy_dir/subdir/file2.txt").exists());
/// assert_eq!(read("copy_dir/file1.txt").unwrap(), "Hello");
/// assert_eq!(read("copy_dir/subdir/file2.txt").unwrap(), "World");
/// ```
pub fn recursive_copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    if from.is_dir() {
        if !to.exists() {
            fs::create_dir_all(to)?;
        }

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let new_from = from.join(entry.file_name());
            let new_to = to.join(entry.file_name());

            if file_type.is_dir() {
                recursive_copy(new_from, new_to)?;
            } else {
                fs::copy(new_from, new_to)?;
            }
        }
    } else {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(from, to)?;
    }

    Ok(())
}

/// Finds files in a directory (and its subdirectories) that match a given predicate.
///
/// # Arguments
///
/// * `path` - The path of the directory to search.
/// * `filter` - A function that takes a `&DirEntry` and returns a `bool`.
///
/// # Returns
///
/// Returns a `Result` containing a `Vec<PathBuf>` of matching files, or a `FileError`.
///
/// # Examples
///
/// ```
/// use dev_utils::file::{create, find};
///
/// create("dir/file1.txt", "").unwrap();
/// create("dir/file2.dat", "").unwrap();
/// create("dir/subdir/file3.txt", "").unwrap();
/// let txt_files = find("dir", |entry| {
///     entry.path().extension().map_or(false, |ext| ext == "txt")
/// }).unwrap();
/// assert_eq!(txt_files.len(), 2);
/// ```
pub fn find<P: AsRef<Path>, F>(path: P, filter: F) -> Result<Vec<PathBuf>>
where
    F: Fn(&DirEntry) -> bool,
{
    let mut results = Vec::new();
    find_internal(path.as_ref(), &filter, &mut results)?;
    Ok(results)
}

// Internal helper function for `find`
fn find_internal<F>(path: &Path, filter: &F, results: &mut Vec<PathBuf>) -> io::Result<()>
where
    F: Fn(&DirEntry) -> bool,
{
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_internal(&path, filter, results)?;
            } else if filter(&entry) {
                results.push(path);
            }
        }
    }
    Ok(())
}

// todo: Check why the test module is not working as expected...
// todo:     The individual tests are working fine but when running the module test, it's not working
// todo:     -> (cargo test --lib file)
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_DIR: &str = "test_file_ops";
    const TEST_FILE: &str = "test_file.txt";
    const TEST_FILE_COPY: &str = "test_file_copy.txt";
    const TEST_FILE_MOVE: &str = "test_file_move.txt";
    const TEST_FILE_RENAME: &str = "test_file_rename.txt";

    fn setup() {fs::create_dir(TEST_DIR);}
    fn cleanup() {fs::remove_dir_all(TEST_DIR);}
    fn get_test_path(filename: &str) -> PathBuf {Path::new(TEST_DIR).join(filename)}

    #[test]
    fn test_crud_operations() {
        setup();

        // Create
        let file_path = get_test_path(TEST_FILE);
        let content = "Hello, World!";
        let created_path = create(&file_path, content).unwrap();
        assert_eq!(created_path, file_path);
        assert!(file_path.exists());

        // Read
        let read_content = read(&file_path).unwrap();
        assert_eq!(read_content, content);

        // Update
        let new_content = "Updated content";
        update(&file_path, new_content).unwrap();
        let updated_content = read(&file_path).unwrap();
        assert_eq!(updated_content, new_content);

        // Append
        let append_content = " Appended content";
        append(&file_path, append_content).unwrap();
        let final_content = read(&file_path).unwrap();
        assert_eq!(final_content, format!("{}{}", new_content, append_content));

        // Delete
        delete(&file_path).unwrap();
        assert!(!file_path.exists());

        cleanup();
    }

    #[test]
    fn test_list_and_find() {
        setup();

        let file_path = get_test_path(TEST_FILE);
        create(&file_path, "Content").unwrap();
        create(&get_test_path("file2.txt"), "Content").unwrap();
        create(&get_test_path("file3.dat"), "Content").unwrap();

        // List
        let entries = list(TEST_DIR).unwrap();
        assert_eq!(entries.len(), 3);

        // Find
        let txt_files = find(TEST_DIR, |entry| {
            entry.path().extension().map_or(false, |ext| ext == "txt")
        }).unwrap();
        assert_eq!(txt_files.len(), 2);

        cleanup();
    }

    #[test]
    fn test_copy_move_rename() {
        setup();

        let original_path = get_test_path(TEST_FILE);
        let copy_path = get_test_path(TEST_FILE_COPY);
        let move_path = get_test_path(TEST_FILE_MOVE);
        let rename_path = get_test_path(TEST_FILE_RENAME);

        // Create original file
        create(&original_path, "Original content").unwrap();

        // Copy
        copy(&original_path, &copy_path).unwrap();
        assert!(copy_path.exists());
        assert_eq!(read(&original_path).unwrap(), read(&copy_path).unwrap());

        // Move
        mv(&copy_path, &move_path).unwrap();
        assert!(!copy_path.exists());
        assert!(move_path.exists());

        // Rename
        rename(&move_path, &rename_path).unwrap();
        assert!(!move_path.exists());
        assert!(rename_path.exists());

        cleanup();
    }

    #[test]
    fn test_recursive_copy() {
        setup();

        let sub_dir = Path::new(TEST_DIR).join("sub_dir");
        fs::create_dir(&sub_dir).unwrap();

        create(&sub_dir.join("file1.txt"), "Content 1").unwrap();
        create(&sub_dir.join("file2.txt"), "Content 2").unwrap();

        let copy_dir = Path::new(TEST_DIR).join("copy_dir");

        recursive_copy(&sub_dir, &copy_dir).unwrap();

        assert!(copy_dir.exists());
        assert!(copy_dir.join("file1.txt").exists());
        assert!(copy_dir.join("file2.txt").exists());

        assert_eq!(read(&copy_dir.join("file1.txt")).unwrap(), "Content 1");
        assert_eq!(read(&copy_dir.join("file2.txt")).unwrap(), "Content 2");

        cleanup();
    }

    #[test]
    fn test_error_handling() {
        // Test non-existent file
        let result = read("non_existent_file.txt");
        assert!(matches!(result, Err(FileError::Io(_))));

        // Test deleting non-existent file
        let result = delete("non_existent_file.txt");
        assert!(matches!(result, Err(FileError::Io(_))));
    }
}
