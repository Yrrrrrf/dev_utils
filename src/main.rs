#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// External crates
use log::LevelFilter;

// Internal modules
use dev_utils::{
    print_app_data,
    log::rlog::RLog,
};


// Main function
fn main() {
    // print_app_data();  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // Print some messages usibng the terminal module with all colors
    // println!("{}", set_bg("Hello, Red Background!", "r"));
    // println!("{}", set_bg("World in Green Background!", "green"));
    // println!("{}", set_bg("Hello, Blue Background!", "b"));
    // println!("{}", set_bg("World in Cyan Background!", "c"));
    // println!("{}", set_bg("Hello, Magenta Background!", "m"));
    // println!("{}", set_bg("World in Yellow Background!", "y"));
    // println!("{}", set_bg("Hello, Default Background!", "defdadault"));

    // println!("{}", set_fg("Hello, Red!", "r"));
    // println!("{}", set_fg("World in Green!", "green"));
    // println!("{}", set_fg("Hello, Blue!", "b"));
    // println!("{}", set_fg("World in Cyan!", "c"));
    // println!("{}", set_fg("Hello, Magenta!", "m"));
    // println!("{}", set_fg("World in Yellow!", "y"));
    // println!("{}", set_fg("Hello, Default!", "defdadault"));

}
