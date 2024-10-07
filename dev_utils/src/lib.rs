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
//! fn main() {
//!     app_dt!(file!());  // Print package name and version from Cargo.toml
//! 
//!     app_dt!(file!(),  // Print package name, version, license, and keywords
//!         "package" => ["license", "keywords"]  // selected sections and keys
//!     );
//! }
//! ```
#![allow(unused)]


pub mod dlog;
pub mod format;
pub mod file;
pub mod datetime;
pub mod base_change;

use std::io::{self, Write};
use std::str::FromStr;
use std::fmt::Display;

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
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = input.trim();

    if trimmed.is_empty() {return Ok(T::default());}

    trimmed.parse().map_err(|e| format!("Parse error: {}", e))
}

pub fn __delay_ms(ms: u64) {std::thread::sleep(std::time::Duration::from_millis(ms));}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    fn test_base_change() {
        vec![  // vec![src_base, new_base, src, result]
            // bin -> dec
            (2, 10, "11011100", "220"),
            (2, 10, "110011", "51"),
            (2, 10, "11001100", "204"),
            (2, 10, "11110011", "243"),
            (2, 10, "1100111", "103"),
            // dec -> bin
            (10, 2, "197", "11000101"),
            (10, 2, "253", "11111101"),
            (10, 2, "79", "1001111"),
            (10, 2, "297", "100101001"),
            (10, 2, "528", "1000010000"),
            // bin -> hex
            (2, 16, "100111011", "13B"),
            (2, 16, "11011011", "DB"),
            (2, 16, "101111011", "17B"),
            (2, 16, "11011001", "D9"),
            (2, 16, "111011101", "1DD"),
            // hex -> bin
            (16, 2, "9F", "10011111"),
            (16, 2, "9BAF", "1001101110101111"),
            (16, 2, "8BCD", "1000101111001101"),
            (16, 2, "72BA", "111001010111010"),
            (16, 2, "987", "100110000111"),
            (16, 2, "9F27", "1001111100100111"),
            // bin -> oct
            (2, 8, "11011001", "331"),
            (2, 8, "100111001", "471"),
            (2, 8, "11100110", "346"),
            (2, 8, "11001100", "314"),
            (2, 8, "1101110", "156"),
            // oct -> bin
            (8, 2, "245", "10100101"),
            (8, 2, "327", "11010111"),
            (8, 2, "651", "110101001"),

            // ? Decimal numbers test
            // These aproximate numbers are not exact because of the floating point precision
            // So the result is not exact, but it's close enough
            // The str_to_num_from_base() fn returns the last number that is not 0. So the result is not exact
            // &Example: 0.102000 -> 0.102 (the last 0s are not returned)
            // TODO: FIX THE DECIMAL PART FUNCTIONS TO COMPARE THIS KIND OF NUMBERS
            // (10, 2, "450.5", "111000010.1"),
            // (10, 2, "8.5", "1000.1"),
            // (10, 8, "450.5", "702.4"),
            // (10, 8, "7.5", "7.4"),
            // (10, 16, "450.5", "1C2.8"),
            // (10, 16, "8.5", "8.8"),
            // (8, 10, "450.5", "296.625"),
            // (8, 10, "7.5", "7.625"),
            // (2, 10, "1010.1", "10.5"),
            // (20, 6, "AA.21", "550.034050123501235"),
            // (10, 16, "2197.42", "895.6B851EB851EB851"),
            // (16, 10, "9E.D", "158.8125"),

        ].iter().for_each(|(src_base, new_base, src, result)|
            // assert_eq!(str_to_num_from_base(src, *src_base, *new_base).unwrap(), result.to_string()));
            println!("{}", 1)
        );

        // * To print the results in the terminal
        // ].iter().for_each(|(src_base, new_base, src, result)|
        //     println!("{} b{:_>2} = {} b{:_>2}\t{}", src, src_base, 
        //         str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
        //         crate::terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
        // ));
    }

    fn test_read_integer() {}
    fn test_read_string() {}
    fn test_pause() {}
}

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


    pub fn extract_app_data_with_sections<'a>(
        data: &'a str,
        sections: &[(&str, &[&str])]
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
                if sections.iter().any(|&(s, keys)| s == current_section && keys.contains(&key)) {
                    let value = value.trim().trim_matches('"');
                    current_key = key;
                    if value.starts_with('[') && !value.ends_with(']') {
                        multi_line_value = value.to_string();
                    } else {
                        app_data.entry(current_section)
                            .or_insert_with(HashMap::new)
                            .insert(key, value.to_string());
                    }
                }
            } else if !line_without_comment.is_empty() && !multi_line_value.is_empty() {
                multi_line_value.push_str(line_without_comment);
                if line_without_comment.ends_with(']') {
                    app_data.entry(current_section)
                        .or_insert_with(HashMap::new)
                        .insert(current_key, multi_line_value.trim_matches(&['[', ']'][..]).to_string());
                    multi_line_value.clear();
                }
            }
        }
    
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
            package_data.get("name").unwrap().color(Color::new(16, 192, 16)),
            package_data.get("version").unwrap().color(Color::new(8, 64, 224)).style(Style::Italic),
        );

        // Only print additional data if any optional fields were provided
        if all_data.len() > 1 || all_data.get("package").map_or(false, |p| p.len() > 2) {
            print_extracted_data(&all_data, &["name", "version"]);
        }
    }};
}
