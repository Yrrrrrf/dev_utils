//! DEV_UTILS is a collection of utilities for development.
//! 
//! It contains utilities for logging, terminal, files, etc.
//! 
pub mod files;
pub mod datetime;
pub mod rlog;
pub mod terminal;


/// Reads the Cargo.toml file and prints the app data
/// 
/// This is also the same as the package data.
pub fn print_app_data() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal
    print!("{}", terminal::set_fg(&env!("CARGO_PKG_NAME").to_uppercase(), "g"));
    println!(" {}", terminal::set_fg(&format!("V{}", env!("CARGO_PKG_VERSION")), "b"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!();
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
}
