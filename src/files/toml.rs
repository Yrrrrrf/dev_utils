//! TOML file utilities for Rust projects.
//!
//! This module provides utilities for working with `Cargo.toml` files in Rust projects.
//! It includes functions to search for `Cargo.toml` files in directories and subdirectories,
//! and to read and extract package information from them.
//!
//! The `find_cargo_toml_files` function allows you to locate all `Cargo.toml` files
//! starting from a specified directory and its subdirectories. It returns a vector
//! of `PathBuf` representing the paths to the found `Cargo.toml` files.
//!
//! The `read_cargo_toml_file` function reads and processes a `Cargo.toml` file, extracting
//! package information such as name, version, and authors, and prints this information.
//!
//! # Example:
//! ```rust
//! use std::path::Path;
//! use dev_utils::files::toml;
//!
//! // Find all Cargo.toml files in the current directory and its subdirectories
//! let current_dir = std::env::current_dir().unwrap();
//! let cargo_toml_files = toml::find_cargo_toml_files(&current_dir);
//!
//! for cargo_toml_file in cargo_toml_files {
//!     println!("Cargo.toml found at: {:?}", cargo_toml_file);
//!     toml::read_cargo_toml_file(&cargo_toml_file);
//! }
//! ```
//!
//! These functions are **particularly useful when working with Rust Cargo workspaces,
//! allowing you to access information from multiple `Cargo.toml` files in one or more
//! subprojects**.
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};


/// Recursively searches for `Cargo.toml` files in the specified directory and its subdirectories.
///
/// This function explores the directory structure starting from the specified `start_dir`.
/// It searches for `Cargo.toml` files in the specified directory and all of its subdirectories.
///
/// # Arguments
///
/// - `start_dir` - The starting directory for the search.
///
/// # Returns
///
/// A vector of [`PathBuf`] representing the paths to found `Cargo.toml` files.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use dev_utils::files::toml::find_cargo_toml_files;
///
/// // Find all Cargo.toml files in the current directory and its subdirectories
/// let current_dir = std::env::current_dir().unwrap();  // Get the current directory
/// let cargo_toml_files = find_cargo_toml_files(&current_dir);
/// 
/// // Print the paths where Cargo.toml files were found
/// for cargo_toml_file in cargo_toml_files {
///     println!("Cargo.toml found at: {:?}", cargo_toml_file);
/// }
/// ```
pub fn find_cargo_toml_files(start_dir: &Path) -> Vec<PathBuf> {
    let mut cargo_toml_files = Vec::new();  // Vector to store the paths to the found Cargo.toml files

    if start_dir.is_dir() {  // If the start_dir is a directory
        for entry in fs::read_dir(start_dir).unwrap() {  // Iterate over the entries in the directory
            let path = entry.unwrap().path();  // Get the path of the entry
            // If the entry is a file and its name is "Cargo.toml", add it to the vector
            match path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
                true => cargo_toml_files.push(path.clone()),  // Add the path to the vector
                // Else, if the entry is a dir, search for Cargo.toml files in it and add them to the vec
                false => if path.is_dir() {cargo_toml_files.extend(find_cargo_toml_files(&path));}
            }
        }
    }
    cargo_toml_files  // Return the cargo_toml_files vector (it will be empty if the start_dir is not a directory)
}


/// Reads a `Cargo.toml` file, extracts information separated by sections, and returns it as a structured data.
///
/// This function parses a `Cargo.toml` file, extracting key-value pairs organized by sections (e.g., "package", "dependencies").
///
/// # Arguments
///
/// - `cargo_toml_path` - The path to the `Cargo.toml` file to be processed.
///
/// # Returns
///
/// A [`HashMap`] where keys represent sections (e.g., "package", "dependencies") and values are
/// sub-maps containing key-value pairs for each section. The top-level keys in the [`HashMap`] correspond
/// to section names, and the associated sub-maps contain the key-value pairs within each section.
///
/// # Example
/// ```
/// use dev_utils::files::toml::read_cargo_toml_file;
/// use std::collections::HashMap;
/// use std::path::Path;
/// 
/// let cargo_toml_path = Path::new("Cargo.toml");
/// let toml_data = read_cargo_toml_file(&cargo_toml_path);
///
/// if let Some(package_info) = toml_data.get("package") {
///     if let Some(name) = package_info.get("name") {
///         println!("Package name: {}", name);
///     }
/// }
/// ```
pub fn read_cargo_toml_file(cargo_toml_path: &Path) -> HashMap<String, HashMap<String, String>> {
    let mut cargo_toml_data = HashMap::new();
    let mut current_section = String::new();

    if let Ok(file) = fs::File::open(cargo_toml_path) {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            let line = line.unwrap();  // Unwrap the line

            if line.starts_with("[") {
                // Extract section name from line
                current_section = line.trim_start_matches('[').trim_end_matches(']').to_string();
                // Create a sub-map for the section if it doesn't exist
                cargo_toml_data
                    .entry(current_section.clone())
                    .or_insert_with(HashMap::new);
            } else if let Some((key, value)) = extract_key_value(&line) {
                // Insert key-value pair into the section's sub-map
                cargo_toml_data
                    .entry(current_section.clone())
                    .and_modify(|map| {
                        map.insert(key, value);
                    });
            }
        }
    }
    cargo_toml_data
}


/// Helper function to extract key-value pairs from a line of a TOML configuration.
///
/// This function parses a line containing a key-value pair in TOML format and returns it as a tuple.
///
/// # Arguments
///
/// - `line` - A string containing a key-value pair in TOML format.
///
/// # Returns
///
/// An [`Option`] containing a tuple with two elements: the key (as a `String`) and the value (as a `String`).
///
/// If the input line is not in a valid key-value format, `None` is returned.
///
/// # Example
///
/// ```
/// use dev_utils::files::toml::extract_key_value;
/// 
/// let line = "name = \"example\"";  // String containing a key-value pair in TOML format
///
/// if let Some((key, value)) = extract_key_value(line) {
///     assert_eq!(key, "name");
///     assert_eq!(value, "example");
/// }
/// ```
pub fn extract_key_value(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();

    match parts.len() {
        2 => {
            let key = parts[0].trim();  // Remove leading/trailing whitespace from the key
            let value = parts[1].trim().trim_matches('"').to_string();  // Remove quotes from the value
            Some((key.to_string(), value))  // Return the key-value pair as a tuple
        }
        _ => None,  // If the line is not in a valid key-value format, return None
    }
}


// todo: Think if this function is needed
// todo: probably needs to be refactored to something like "get package data"
// todo: or maybe it should be a method of the CargoFile struct
/// Reads and processes a `Cargo.toml` file, extracting package information.
/// 
/// This function reads the specified `Cargo.toml` file and extracts package information
///
/// # Arguments
///
/// - `cargo_toml_path` - The path to the `Cargo.toml` file to be processed.
pub fn print_package_data(cargo_toml_path: &Path) {
    // let mut name = String::new();
    // let mut version = String::new();
    // let mut authors = String::new();

    // GET THE PACKAGE NAME, VERSION, AND AUTHORS
    let cargo_file = read_cargo_toml_file(&cargo_toml_path);
    if let Some(package_info) = cargo_file.get("package") {
        if let Some(name) = package_info.get("name") {println!("Package name: {}", name);}
        if let Some(version) = package_info.get("version") {println!("Package version: {}", version);}
        if let Some(authors) = package_info.get("authors") {println!("Package authors: {}", authors);}
        println!();
    }
}
