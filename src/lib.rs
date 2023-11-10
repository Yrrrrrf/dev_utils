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
//!     print_app_data(file!());  // Print application data (name, version, authors, etc.)
//!     RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level
//!     log::info!("Some data!");  // [2021-01-01 00:00:00] INFO: Hello World!
//!     // Your code here...
//! }
//! ```

use std::{path::Path, collections::HashMap};
#[allow(unused)]

pub mod log;
pub mod files;
pub mod conversion;
pub mod console;

// ^ Still need to add the following modules:
mod codex;
mod crypto;


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
pub fn print_app_data(actual_file_path: &'static str) {
    // #[allow(unused)]
    use files::toml::*;  // Import the toml module (Only when using the print_app_data() fn)
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal

    let start_dir = Path::new(actual_file_path)
        .parent().unwrap()  // Get the src directory
        .parent().unwrap();  // Get the project directory (can be a workspace or a single project)
    let mut package_info: &HashMap<String, String> = &HashMap::new();
    let mut cargo_file: Option<CargoFile>;

    for entry in std::fs::read_dir(start_dir).unwrap() {  // Iterate over the entries in the directory
        let path = entry.unwrap().path();  // Get the path of the entry
        if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            cargo_file = Some(CargoFile::new(&path));
            package_info = cargo_file.as_ref().unwrap().get_section_data("package").unwrap();
            // println!("{:#?}\n", package_info);
        }
    }

    println!("\x1b[32m{}\x1b[0m \x1b[34m{}\x1b[0m", 
        package_info.get("name").unwrap(), 
        format!("V{}", package_info.get("version").unwrap())
    );
    println!("Authors: {}\n", package_info.get("authors").unwrap());
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
