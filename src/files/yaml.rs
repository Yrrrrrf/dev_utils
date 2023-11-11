//! YAML file utilities for Rust projects.
//!
//! This module provides utilities for working with YAML files in Rust projects.
//! It includes functions to search for YAML files in directories and subdirectories,
//! and to read and extract information from them.
//!
//! The `find_yaml_files` function allows you to locate all YAML files
//! starting from a specified directory and its subdirectories. It returns a vector
//! of `PathBuf` representing the paths to the found YAML files.
//!
//! The `read_yaml_file` function reads and processes a YAML file, extracting
//! information, and prints this information.
//!
//! These functions are **particularly useful when working with configuration files
//! or data files in YAML format**.
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};


/// Recursively searches for YAML files in the specified directory and its subdirectories.
///
/// This function explores the directory structure starting from the specified `start_dir`.
/// It searches for YAML files in the specified directory and all of its subdirectories.
///
/// # Arguments
///
/// - `start_dir` - The starting directory for the search.
///
/// # Returns
///
/// A vector of [`PathBuf`] representing the paths to found YAML files.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use dev_utils::files::yaml::find_yaml_files;
///
/// // Find all YAML files in the current directory and its subdirectories
/// let current_dir = std::env::current_dir().unwrap();  // Get the current directory
/// let yaml_files = find_yaml_files(&current_dir);
/// 
/// // Print the paths where YAML files were found
/// for yaml_file in yaml_files {
///     println!("YAML file found at: {:?}", yaml_file);
/// }
/// ```
pub fn find_yaml_files(start_dir: &Path) -> Vec<PathBuf> {
    let mut yaml_files = Vec::new();

    if start_dir.is_dir() {
        for entry in fs::read_dir(start_dir).unwrap() {
            let path = entry.unwrap().path();
            match path.is_file() && path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                true => yaml_files.push(path.clone()),
                false => if path.is_dir() { yaml_files.extend(find_yaml_files(&path)); }
            }
        }
    }
    yaml_files
}

/// This struct represents a YAML file and provides methods to read and extract information from it.
/// 
/// Used to read and extract information from a YAML file.
#[derive(Debug, Clone, PartialEq)]
pub struct YamlFile {
    pub path: PathBuf,
    // pub data: HashMap<String, serde_yaml::Value>,
}

impl YamlFile {
    /// Creates a new `YamlFile` instance by reading the specified YAML file.
    ///
    /// This function initializes a `YamlFile` instance by reading the YAML file located at the given `path`.
    /// It also extracts and structures the data within the YAML file.
    ///
    /// # Arguments
    ///
    /// - `path` - The path to the YAML file.
    ///
    /// # Returns
    ///
    /// A `YamlFile` instance containing the YAML file path and structured data.
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            // data: Self::get_yaml_data(path),
        }
    }

    // /// Reads a YAML file, extracts information, and returns it as a structured data.
    // ///
    // /// This function parses a YAML file, extracting key-value pairs.
    // ///
    // /// # Arguments
    // ///
    // /// - `yaml_path` - The path to the YAML file to be processed.
    // ///
    // /// # Returns
    // ///
    // /// A [`HashMap`] where keys represent keys in the YAML file, and values are
    // /// associated [`serde_yaml::Value`] instances.
    // pub fn get_yaml_data(yaml_path: &Path) -> HashMap<String, serde_yaml::Value> {
    //     let yaml_content = fs::read_to_string(yaml_path).unwrap();
    //     serde_yaml::from_str(&yaml_content).unwrap()
    // }

    // /// Gets a specific value from the YAML file.
    // ///
    // /// # Arguments
    // ///
    // /// - `key` - The key to retrieve.
    // ///
    // /// # Returns
    // ///
    // /// An `Option` containing a reference to the value as a [`serde_yaml::Value`].
    // pub fn get_value(&self, key: &str) -> Option<&serde_yaml::Value> {
    //     self.data.get(key)
    // }
}

