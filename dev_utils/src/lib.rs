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



// dlog should export the macros, so it adds the following line:

pub mod dlog;
pub mod format;


// pub mod http;
pub mod files;
// pub mod console;
pub mod conversion;




use std::collections::HashMap;

/// Module containing helper functions for the print_app_data macro
pub mod helpers {
    use std::path::PathBuf;
    use std::fs;
    use std::io;
    use std::env;
    use std::collections::HashMap;

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
    ) -> HashMap<&'a str, HashMap<&'a str, &'a str>> {
        let mut app_data = HashMap::new();
        let mut current_section = "";

        for line in data.lines() {
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line.trim_matches(&['[', ']'][..]);
            } else if let Some((key, value)) = line.split_once('=') {
                if let Some((_, keys)) = sections.iter().find(|&&(s, _)| s == current_section) {
                    let key = key.trim();
                    if keys.contains(&key) {
                        let value = value.trim().trim_matches('"');
                        app_data.entry(current_section)
                            .or_insert_with(HashMap::new)
                            .insert(key, value);
                    }
                }
            }
        }

        app_data
    }

    /// Prints the extracted data in a formatted manner.
    pub fn print_extracted_data(app_data: &HashMap<&str, HashMap<&str, &str>>) {
        for (section, data) in app_data {
            // println!("[{}]", console::style(section).yellow().bold());
            println!("[{section}]");
            for (key, value) in data {
                // println!("{}: {}", console::style(key).BLUE(), value);
                println!("{key}: {value}");
            }
            println!();
        }
    }
}

/// Macro for printing application data extracted from Cargo.toml
///
/// This macro clears the terminal screen, extracts relevant information from the 'Cargo.toml' file,
/// and prints this information in a structured format. It always displays the application name and version
/// in color, and optionally extracts and displays additional specified sections and keys.
///
/// # Usage
///
/// ```rust
/// // Basic usage (only prints name and version)
/// print_app_data!(file!());
///
/// // With additional sections to extract
/// print_app_data!(file!(), 
///     "package" => ["authors", "description"],
///     "dependencies" => ["serde", "tokio"]
/// );
/// ```
///
/// # Arguments
///
/// * `$file_path` - A literal representing the path of the file calling this macro (usually `file!()`)
/// * `$($section:expr => [$($key:expr),+ $(,)?]),*` - Optional: Sections and keys to extract from Cargo.toml
///
/// # Panics
///
/// This macro will panic if:
/// - The 'Cargo.toml' file cannot be found in any parent directory.
/// - There are issues reading or parsing the 'Cargo.toml' file.
/// - The "package" section or the "name" and "version" keys are not found in the 'Cargo.toml' file.
#[macro_export]
macro_rules! app_dt {
    ($file_path:expr $(, $($section:expr => [$($key:expr),+ $(,)?]),* $(,)?)?) => {{
        use std::io::Write;
        use $crate::helpers::{find_cargo_toml, extract_app_data_with_sections, print_extracted_data};

        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H");
        let _ = std::io::stdout().flush();

        // Find and read Cargo.toml
        let cargo_toml_path = find_cargo_toml($file_path).expect("Failed to find Cargo.toml");
        let cargo_toml = std::fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

        // Extract required "package" section data
        let mut required_data = extract_app_data_with_sections(&cargo_toml, &[("package", &["name", "version"])]);
        let package_data = required_data.get("package").expect("Failed to extract package data");
        
        // Print name and version in color
        let name = package_data.get("name").expect("Failed to extract package name");
        let version = package_data.get("version").expect("Failed to extract package version");
        println!("{name} {version}\n");

        // Extract and print additional sections if specified
        $(
            let additional_data = extract_app_data_with_sections(&cargo_toml, &[$((stringify!($section), &[$($key),+] as &[&str])),*]);
            print_extracted_data(&additional_data);
        )?
    }};
}



#[cfg(test)]
mod tests {
    use format::*;

    use super::*;

    #[test]
    fn test_color_creation() {
        assert_eq!(Color::RED.to_rgb(), RGB(255, 0, 0));
        assert_eq!(Color::from((128, 64, 32)).to_rgb(), RGB(128, 64, 32));
    }

    #[test]
    fn test_color_codes() {
        assert_eq!(Color::BLUE.fg_code(), "\x1b[38;2;0;0;255m");
        assert_eq!(Color::GREEN.bg_code(), "\x1b[48;2;0;255;0m");
        assert_eq!(Color::from((128, 64, 32)).fg_code(), "\x1b[38;2;128;64;32m");
    }

    #[test]
    fn test_style_codes() {
        assert_eq!(Style::Bold.code(), "\x1b[1m");
        assert_eq!(Style::Underline.code(), "\x1b[4m");
    }

    #[test]
    fn test_stylize_trait() {
        let text = "Test";
        assert_eq!(text.color(Color::RED), "\x1b[38;2;255;0;0mTest\x1b[0m");
        assert_eq!(text.on_color(Color::BLUE), "\x1b[48;2;0;0;255mTest\x1b[0m");
        assert_eq!(text.style(Style::Bold), "\x1b[1mTest\x1b[0m");
    }

    #[test]
    fn test_visual_length() {
        let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGREEN\x1b[0m";
        assert_eq!(visual_length(colored_text), 9); // "RED GREEN".len()
    }

    #[test]
    fn test_complex_formatting() {
        let text = "Complex";
        let formatted = text.color(Color::RED).on_color(Color::WHITE).style(Style::Bold);
        let stripped = strip_ansi_codes(&formatted);
        assert_eq!(stripped, "Complex");
        assert!(formatted.contains("\x1b[38;2;255;0;0m")); // Red foreground
        assert!(formatted.contains("\x1b[48;2;255;255;255m")); // White background
        assert!(formatted.contains("\x1b[1m")); // Bold
        assert!(formatted.ends_with("\x1b[0m")); // Reset at the end
    }

    #[test]
    fn test_custom_color() {
        let custom_color = Color::from((123, 45, 67));
        assert_eq!(custom_color.fg_code(), "\x1b[38;2;123;45;67m");
    }

    #[test]
    fn test_strip_ansi_codes() {
        let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m";
        assert_eq!(strip_ansi_codes(colored_text), "Red Green");
        
        // Test with multiple consecutive ANSI codes
        let multi_code = "\x1b[31;1mBold Red\x1b[0m";
        assert_eq!(strip_ansi_codes(multi_code), "Bold Red");
    }
    
    #[test]
    fn test_incomplete_ansi_sequence() {
        let incomplete = "Normal \x1b[31m Red \x1b[ Incomplete";
        assert_eq!(strip_ansi_codes(incomplete), "Normal  Red  Incomplete");
        
        // Test with incomplete sequence at the end
        let incomplete_end = "Text with incomplete sequence\x1b[";
        assert_eq!(strip_ansi_codes(incomplete_end), "Text with incomplete sequence");
    }
}
