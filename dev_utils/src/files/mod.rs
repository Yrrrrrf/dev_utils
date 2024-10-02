//! Some of the most common file operations to manipulate files and directories.

#![allow(unused)]

use std::fs;
use std::path::Path;
use std::io::{self, Error};


pub mod crud;
// pub use crud::*;

// *  fs::read_to_string(file_path)
// *  fs::rename(original_path, destination_path)
// *  fs::remove_file(file_path)
// *  fs::rename(original_path, new_path)

