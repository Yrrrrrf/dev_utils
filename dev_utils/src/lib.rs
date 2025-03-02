//! A collection of utility functions for common development tasks, including logging, terminal manipulation, file handling, and more.
//!
//! ```toml
//! [dependencies]
//! dev_utils = "0.*"  # Add the latest version of this crate
//! ```
//!
//! # Usage
//!
//! ```rust
//! use dev_utils::app_dt;
//!
//! app_dt!(file!());  // Print package name and version from Cargo.toml
//! app_dt!(file!(),  // Print package name, version, license, and keywords
//!     "package" => ["license", "keywords"]  // selected sections and keys
//! );
//! ```
#![allow(unused)]

pub mod base_change;
pub mod datetime;
pub mod dlog;
pub mod file;
pub mod format;

use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;

// todo: FOR dev_utils: dev_macros/some(mod)
// todo:     - some custom proc-macro to gen:
// todo:         - new() fn should have default values for all fields
// todo:         - new(*args) fn should have custom values for all fields

/// Reads input from the console, optionally displaying a prompt message.
///
/// This function can:
/// - Display a custom prompt message
/// - Read input until the user presses Enter
/// - Parse the input into any type that implements FromStr
/// - Act as a pause mechanism when no prompt is provided
///
/// # Type Parameters
///
/// - `T`: The type to parse the input into. Must implement FromStr and Default.
///
/// # Arguments
///
/// - `prompt`: An optional prompt message to display before reading input.
///
/// # Returns
///
/// - `T`: The parsed input value
/// - `String`: The raw input string if parsing fails or no parsing is needed
///
/// # Examples
///
/// ```
/// let number: i32 = read_input(Some("Enter a number: ")).unwrap();
/// let name: String = read_input(Some("Enter your name: ")).unwrap();
/// read_input::<String>(None); // Acts as a pause
/// ```
pub fn read_input<T>(prompt: Option<&str>) -> Result<T, String>
where
    T: FromStr + Default,
    <T as FromStr>::Err: Display,
{
    if let Some(msg) = prompt {
        print!("{}", msg);
        io::stdout().flush().unwrap();
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(T::default());
    }

    trimmed.parse().map_err(|e| format!("Parse error: {}", e))
}

/// Delays the program execution for the specified number of milliseconds.
pub fn __delay_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

/// Module containing helper functions for the print_app_data macro
pub mod helpers {
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

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
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Cargo.toml not found in any parent directory",
                ));
            }
        }
    }

    pub fn extract_app_data_with_sections<'a>(
        data: &'a str,
        sections: &[(&str, &[&str])],
    ) -> HashMap<&'a str, HashMap<&'a str, String>> {
        let mut app_data = HashMap::new();
        let mut current_section = "";
        let mut current_key = "";
        let mut multi_line_value = String::new();

        for line in data.lines() {
            let trimmed_line = line.trim();

            // Skip empty lines and full-line comments
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            // Remove inline comments
            let line_without_comment = trimmed_line.split('#').next().unwrap().trim();

            if line_without_comment.starts_with('[') && line_without_comment.ends_with(']') {
                current_section = line_without_comment.trim_matches(&['[', ']'][..]);
            } else if let Some((key, value)) = line_without_comment.split_once('=') {
                let key = key.trim();
                if sections
                    .iter()
                    .any(|&(s, keys)| s == current_section && keys.contains(&key))
                {
                    let value = value.trim().trim_matches('"');
                    current_key = key;
                    if value.starts_with('[') && !value.ends_with(']') {
                        multi_line_value = value.to_string();
                    } else {
                        app_data
                            .entry(current_section)
                            .or_insert_with(HashMap::new)
                            .insert(key, value.to_string());
                    }
                }
            } else if !line_without_comment.is_empty() && !multi_line_value.is_empty() {
                multi_line_value.push_str(line_without_comment);
                if line_without_comment.ends_with(']') {
                    app_data
                        .entry(current_section)
                        .or_insert_with(HashMap::new)
                        .insert(
                            current_key,
                            multi_line_value.trim_matches(&['[', ']'][..]).to_string(),
                        );
                    multi_line_value.clear();
                }
            }
        }

        app_data
    }

    pub fn print_extracted_data(
        app_data: &HashMap<&str, HashMap<&str, String>>,
        skip_keys: &[&str],
    ) {
        for (section, data) in app_data {
            println!("{}:", section.style(Style::Bold));
            for (key, value) in data {
                if !skip_keys.contains(key) {
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
            package_data.get("name").unwrap().color(Color::new(16, 192, 16)),
            package_data.get("version").unwrap().color(Color::new(8, 64, 224)).style(Style::Italic),
        );

        // Only print additional data if any optional fields were provided
        if all_data.len() > 1 || all_data.get("package").map_or(false, |p| p.len() > 2) {
            print_extracted_data(&all_data, &["name", "version"]);
        }
    }};
}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_useful_test() {
        app_dt!(file!()); // Print package name and version from Cargo.toml
    }
}
