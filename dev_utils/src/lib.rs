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


use std::path::Path;
use std::fs;

pub mod log;
// pub mod http;
// pub mod files;
pub mod console;
pub mod conversion;

/// Clears the terminal screen and extracts relevant information from the 'Cargo.toml' file,
/// then prints this information in a structured format.
///
/// This function clears the terminal screen using ANSI escape codes and reads the 'Cargo.toml' file
/// to retrieve information such as the application name, version, and authors. It then prints this
/// information to the terminal.
///
/// # Examples
///
/// To use this function in your Rust application, you can import it and call it within your `main` function:
///
/// ```rust
/// use dev_utils::print_app_data;
///
/// fn main() {
///     print_app_data(file!());  // Print application data
/// }
/// ```
///
/// The function will clear the terminal and display information extracted from 'Cargo.toml', such as:
///
/// ```plaintext
/// MyApplicationName V0.1.0
/// Authors: Some Author, Another Author
/// ```

pub fn print_app_data(
    actual_file_path: &'static str
) {
    print!("\x1B[2J \x1B[1;1H");  // Clear the terminal screen

    let mut path = Path::new(actual_file_path);

    let file_name = "Cargo.toml";
    // let file_name = "TheFile.toml";

    let mut cargo_toml = String::new();
    let mut toml_is_found = false;
    
    while !toml_is_found {
        path = path.parent().unwrap();  // Go to the parent directory
        // println!("\npath: {}", path.display());

        if path.to_str().unwrap() == "" {
            // print all files in the current directory
            // fs::read_dir(".").unwrap().for_each(|entry| println!("\tfile: {}", entry.unwrap().path().display()));
            cargo_toml = fs::read_to_string(file_name).unwrap();
            break;
        };

        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            // println!("\tfile: {}", path.display());
            if path.file_name().unwrap().to_str().unwrap() == file_name {
                cargo_toml = fs::read_to_string(path).unwrap();
                toml_is_found = true;
            }
        }

    }

    let app_data = extract_app_data_with_sections!(cargo_toml, "package" => ["name", "version", "authors"]);
    println!("{} v{}\n",
        console::format::set_fg(app_data.get("package").unwrap().get("name").unwrap().to_string(), 'g'),
        console::format::set_fg(app_data.get("package").unwrap().get("version").unwrap().to_string(), 'b'),
    );

}


#[macro_export]
macro_rules! extract_app_data_with_sections {
    ($data:expr, $($section:expr => [$($key:expr),+]),+) => {{
        let mut app_data: std::collections::HashMap<&str, std::collections::HashMap<&str, &str>> = std::collections::HashMap::new();
        let mut current_section = "";

        for line in $data.lines() {
            if line.starts_with("[") && line.ends_with("]") {
                current_section = line.trim_matches(&['[', ']'][..]);
            } else {
                $(
                    if current_section == $section {
                        $(
                            if line.contains($key) {
                                if let Some(index) = line.find('=') {
                                    let value = line[(index + 1)..].trim().trim_matches('"');
                                    app_data.entry($section).or_insert_with(std::collections::HashMap::new).insert($key, value);
                                }
                            }
                        )*
                    }
                )*
            }
        }
        app_data
    }};
}


// * Same as the macro above, but with a different syntax
// * The main difference is that the macro above uses the `+` operator to match one or more keys, while this one uses the `*` operator to match zero or more keys
// fn extract_app_data_with_sections(data: &str) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
//     let mut app_data = std::collections::HashMap::new();
//     let mut current_section = String::new();

//     for line in data.lines() {
//         if line.starts_with("[") && line.ends_with("]") {
//             current_section = line.trim_matches(&['[', ']'][..]).to_string();
//         } else if let Some(index) = line.find('=') {
//             let key = line[..index].trim();
//             let value = line[(index + 1)..].trim().trim_matches('"');
//             app_data.entry(current_section.clone())
//                     .or_insert_with(std::collections::HashMap::new)
//                     .insert(key.to_string(), value.to_string());
//         }
//     }

//     app_data
// }
