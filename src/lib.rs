//! DEV_UTILS is a collection of utilities for development.
//! 
//! It contains utilities for logging, terminal, files, etc.
//! 
//! # Installation
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! dev_utils = "0.*"  # Add the latest version of this crate
//! log = "0.4"  # This crate also depends on the log crate, so add it too
//! ```
//! 
//! # Usage
//! 
//! ```rust
//! use dev_utils::print_app_data;
//! use dev_utils::rlog::RLog;  // Logger (RLog) from this crate
//! use log::LevelFilter;  // Log crate
//! 
//! fn main() {
//!     print_app_data();  // Print application data
//!     RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level
//!     log::info!("Some data!");  // [2021-01-01 00:00:00] INFO: Hello World!
//!     // Your code here...
//! }
//! ```
pub mod files;
pub mod datetime;
pub mod rlog;
pub mod terminal;

// ^ Still need to add the following modules:
mod log;  // To replace the log crate


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
///     print_app_data();
/// }
/// ```
///
/// The function will clear the terminal and display information extracted from 'Cargo.toml', such as:
///
/// ```plaintext
/// MyApplicationName
/// V1.0.0
/// Authors: John Doe, Jane Smith
/// ```
pub fn print_app_data() {
    // Imports for reading the Cargo.toml file
    use std::fs;
    use std::io::{self, BufRead};
    use std::path::Path;

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal

    if let Ok(file) = fs::File::open(&Path::new("Cargo.toml")) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                match line {
                    _ if line.starts_with("name =") => print!("{}", terminal::set_fg(&line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase(), 'g')),
                    _ if line.starts_with("version =") => println!(" {}", terminal::set_fg(&format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", "")), 'b')),
                    _ if line.starts_with("authors =") => println!("Authors: {}\n", line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", "")),
                    _ => (),  // Else do nothing
                }
            }
        }
    }
}


/// This section contains the tests for the library.
/// 
/// There also doc tests in the source code. But this section is for unit tests.
#[cfg(test)]
mod tests {
    // use super::*;  // use everything in the parent module
    // todo: Impl unit tests for the lib

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    // #[test]
    // fn test_log_init() {
    //     log::info!("log is't initialized, you can't see me");
    //     log_init();
    //     log::info!("log is initialized");
    // }

    // #[test]
    // fn test_log_init_with_level() {
    //     log::info!("log is't initialized, you can't see me");
    //     log_init_with_level(Level::TRACE);
    //     log::trace!("log is initialized");
    // }

}
