//! DEV_UTILS is a collection of utilities for development.
//! 
//! It contains utilities for logging, terminal, files, etc.
//!  
//! Add this to your `Cargo.toml`:
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
//!     print_app_data();  // Print application data
//!     RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level
//!     log::info!("Some data!");  // [2021-01-01 00:00:00] INFO: Hello World!
//!     // Your code here...
//! }
//! ```
// #[allow(unused)]

pub mod log;
pub mod files;
pub mod conversion;

// ^ Still need to add the following modules:
mod codex;
mod crypto;
mod console;


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
use files::toml::*;


pub fn print_app_data() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal

    find_cargo_toml_files(&std::env::current_dir().unwrap()).iter().for_each(|cargo_toml_file| {
        // println!("Cargo.toml found at: {:?}", cargo_toml_file);
        // print_package_data(&cargo_toml_file);

        // println!("Package name: {:#?}", cargo_file);

        let cargo_file = read_cargo_toml_file(&cargo_toml_file);
        if let Some(package_info) = cargo_file.get("package") {
            if let Some(name) = package_info.get("name") {println!("Package name: {}", name);}
            if let Some(version) = package_info.get("version") {println!("Package version: {}", version);}
            if let Some(authors) = package_info.get("authors") {println!("Package authors: {}", authors);}
            println!();
        }

    });
}


// todo: Impl unit tests for the lib
/// This section contains the tests for the library.
/// 
/// There also doc tests in the source code. But this section is for unit tests.
#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;  // use everything in the parent module


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
