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
pub mod log;
pub mod conversion;
pub mod files;

// ^ Still need to add the following modules:




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
// pub fn print_app_data() {
//     // Imports for reading the Cargo.toml file
//     use std::fs;
//     use std::io::{self, BufRead};
//     use std::path::Path;

//     print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal

//     if let Ok(file) = fs::File::open(&Path::new("Cargo.toml")) {
//         for line in io::BufReader::new(file).lines() {
//             if let Ok(line) = line {
//                 match line {
//                     _ if line.starts_with("name =") => print!("{}", terminal::set_fg(&line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase(), 'g')),
//                     _ if line.starts_with("version =") => println!(" {}", terminal::set_fg(&format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", "")), 'b')),
//                     _ if line.starts_with("authors =") => println!("Authors: {}\n", line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", "")),
//                     _ => (),  // Else do nothing
//                 }
//             }
//         }
//     }
// }


#[allow(unused)]

use files::toml::*;


pub fn print_app_data() {
    let mut name = String::new();
    let mut version = String::new();
    let mut authors = String::new();

    // print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal

    find_cargo_toml_files(&std::env::current_dir().unwrap()).iter().for_each(|cargo_toml_file| {
        println!("Cargo.toml found at: {:?}", cargo_toml_file);
        read_cargo_toml_file(&cargo_toml_file);
    });


    // if let Ok(file) = fs::File::open(&Path::new("Cargo.toml")) {
    //     let mut lines = io::BufReader::new(file).lines().peekable();

    //     while let Some(Ok(line)) = lines.next() {  // While there are still lines to read
            
            
            
            
            
            // if line.starts_with("[package]") {  // If the line starts with "[package]"
            //     while let Some(Ok(line)) = lines.next() {
            //         match line {
            //             _ if line.starts_with("name =") => name = line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase(),
            //             _ if line.starts_with("version =") => version = format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", "")),
            //             _ if line.starts_with("authors =") => authors = line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", ""),
            //             _ if line.starts_with("[") => break,  // Stop when the next section starts
            //             _ => (),  // Else do nothing
            //         }
            //     }
            //     break;
            // }
    //     }
    //     println!("{} {}\nAuthors: {}\n", name, version, authors);
    // }

    // match fs::File::open(&Path::new("Cargo.toml")) {
    //     Ok(file) => {
    //         let mut lines = io::BufReader::new(file).lines().peekable();
    //         while let Some(Ok(line)) = lines.next() {  // While there are still lines to read
    //             if line.starts_with("[package]") {  // If the line starts with "[package]"
    //                 while let Some(Ok(line)) = lines.next() {
    //                     match line {
    //                         _ if line.starts_with("name =") => name = line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase(),
    //                         _ if line.starts_with("version =") => version = format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", "")),
    //                         _ if line.starts_with("authors =") => authors = line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", ""),
    //                         _ if line.starts_with("[") => break,  // Stop when the next section starts
    //                         _ => (),  // Else do nothing
    //                     }
    //                 }
    //                 break;
    //             }
    //         }
    //         println!("{} {}\nAuthors: {}\n", name, version, authors);
    //     }
    //     Err(e) => eprintln!("Failed to open 'Cargo.toml': {}", e),
    // };
}



























// todo: Impl unit tests for the lib
/// This section contains the tests for the library.
/// 
/// There also doc tests in the source code. But this section is for unit tests.
#[cfg(test)]
mod tests {
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



















    

    #[test]  // Indicates that this is a test
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
            assert_eq!(conversion::base_change::str_to_num_from_base(src, *src_base, *new_base).unwrap(), result.to_string()));

        // * To print the results in the terminal
        // ].iter().for_each(|(src_base, new_base, src, result)|
        //     println!("{} b{:_>2} = {} b{:_>2}\t{}", src, src_base, 
        //         str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
        //         crate::terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
        // ));
    }
}
