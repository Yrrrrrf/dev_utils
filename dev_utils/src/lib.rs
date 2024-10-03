//! A collection of utility functions for common development tasks, including logging, terminal manipulation, file handling, and more.
//! 
//! ```toml
//! [dependencies]
//! dev_utils = "0.*"  # Add the latest version of this crate
//! log = "0.*"  # This crate also depends on the log crate, so add it too
//! ```
//! 
//! # Usage
//! 
//! ```rust 
//! use dev_utils::log::print_app_data;
//! use dev_utils::log::rlog::RLog;  // Logger (RLog) from this crate
//! use log::LevelFilter;  // Log crate
//! 
//! fn main() {
//!     print_app_data(file!());  // Print application data (name, version, authors, etc.)
//!     RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level
//!     log::info!("Some data!");  // [2021-01-01 00:00:00] INFO: Hello World!
//!     // Your code here...
//! }
//! ```
#![allow(unused)]




pub mod dlog;
pub mod format;


pub mod crud;
// pub mod console;
pub mod conversion;








// ^ mod io (Input/Output) is not ready yet. So isn't a public module.
// # Console Input/Output Module
// This module provides functions for interacting with the console, including input and output operations.
use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


/// This ask() function is still a prototype, so **it could not work as expected**.
/// 
/// Ask for input from the console
/// 
/// ### Parameters:
/// - `T: std::str::FromStr` - The type of the input
/// 
/// ### Returns:
/// - `T` - The input
pub fn ask<T: std::str::FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<T>().unwrap();
}

// console::clear_screen();
// console::set_color("red");
// console::print("Hello, world!");
// console::reset_color();
// console::print("Hello, world!");
// console::set_color("green");


/// Pause the program until the user presses enter.
/// 
/// This function will print a message to the console and wait for the user to press enter.
#[inline]
fn pause() {
    println!("Press enter to exit...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("Error reading line");
}










use std::collections::HashMap;

/// Module containing helper functions for the print_app_data macro
pub mod helpers {
    use std::path::PathBuf;
    use std::fs;
    use std::io;
    use std::env;
    use std::collections::HashMap;

    use crate::format::{Color, Style, Stylize};

    /// Finds the Cargo.toml file by traversing up the directory tree.
    pub fn find_cargo_toml(start_path: &str) -> io::Result<PathBuf> {
        let mut path = PathBuf::from(start_path);
        loop {
            path = match path.parent() {
                Some(parent) => parent.to_path_buf(),
                None => env::current_dir()?,
            };

            let cargo_path = path.join("Cargo.toml");
            if cargo_path.exists() {
                return Ok(cargo_path);
            }

            if path.as_os_str().is_empty() {
                return Err(io::Error::new(io::ErrorKind::NotFound, "Cargo.toml not found in any parent directory"));
            }
        }
    }

    /// Extracts specified data from Cargo.toml content.
    pub fn extract_app_data_with_sections<'a>(
        data: &'a str,
        sections: &[(&str, &[&str])]
    ) -> HashMap<&'a str, HashMap<&'a str, String>> {
        let mut app_data = HashMap::new();
        let mut current_section = "";
        let mut current_key = "";
        let mut multi_line_value = String::new();
    
        // println!("Sections to extract: {:?}", sections);  // Debug print
    
        for line in data.lines() {
            let trimmed_line = line.trim();
            // println!("Processing line: {}", trimmed_line);  // Debug print
    
            if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
                current_section = trimmed_line.trim_matches(&['[', ']'][..]);
                // println!("Current section: {}", current_section);  // Debug print
            } else if let Some((key, value)) = trimmed_line.split_once('=') {
                let key = key.trim();
                if sections.iter().any(|&(s, keys)| s == current_section && keys.contains(&key)) {
                    let value = value.trim().trim_matches('"');
                    current_key = key;
                    // println!("Found key: {}, value: {}", key, value);  // Debug print
                    if value.starts_with('[') && !value.ends_with(']') {
                        multi_line_value = value.to_string();
                    } else {
                        app_data.entry(current_section)
                            .or_insert_with(HashMap::new)
                            .insert(key, value.to_string());
                    }
                }
            } else if !trimmed_line.is_empty() && !multi_line_value.is_empty() {
                multi_line_value.push_str(trimmed_line);
                if trimmed_line.ends_with(']') {
                    app_data.entry(current_section)
                        .or_insert_with(HashMap::new)
                        .insert(current_key, multi_line_value.trim_matches(&['[', ']'][..]).to_string());
                    multi_line_value.clear();
                    // println!("Inserted multi-line value for key: {}", current_key);  // Debug print
                }
            }
        }
    
        // println!("Extracted data: {:?}", app_data);  // Debug print
        app_data
    }

    pub fn print_extracted_data(app_data: &HashMap<&str, HashMap<&str, String>>, skip_keys: &[&str]) {
        for (section, data) in app_data {
            println!("{}:", section.style(Style::Bold));
            for (key, value) in data {
                if !skip_keys.contains(&key) {
                    println!("\t{key}: {}", value.style(Style::Italic).style(Style::Dim));
                }
            }
            println!();
        }
    }
}

#[macro_export]
macro_rules! app_dt {
    ($file_path:expr $(, $($section:expr => [$($key:expr),+ $(,)?]),* $(,)?)?) => {{
        use std::io::Write;
        use $crate::format::*;
        use $crate::helpers::{find_cargo_toml, extract_app_data_with_sections, print_extracted_data};

        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H");
        let _ = std::io::stdout().flush();

        // Find and read Cargo.toml
        let cargo_toml_path = find_cargo_toml($file_path).expect("Failed to find Cargo.toml");
        let cargo_toml = std::fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

        // Extract all data in a single call
        let all_data = extract_app_data_with_sections(&cargo_toml, &[
            ("package", &["name", "version"]),
            $( $(($section, &[$($key),+])),* )?
        ]);

        let package_data = all_data.get("package").expect("Failed to extract package data");

        println!("{} v{}\n",
            package_data.get("name").unwrap().color(Color::Custom(RGB(16, 192, 16))),
            package_data.get("version").unwrap().color(Color::Custom(RGB(8, 64, 224))).style(Style::Italic),
        );

        // Only print additional data if any optional fields were provided
        if all_data.len() > 1 || all_data.get("package").map_or(false, |p| p.len() > 2) {
            print_extracted_data(&all_data, &["name", "version"]);
        }
    }};
}