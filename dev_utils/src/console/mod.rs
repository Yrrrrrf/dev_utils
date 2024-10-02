//! Terminal Utilities
//! 
//! These are utilities for the terminal.
//! It allow us to change the color of the text in the terminal.
//! 
//! Also have some functions to ask for input from the console.
//! 
#![allow(unused)]  // Allow unused code (temporarily)


// ^ mod io (Input/Output) is not ready yet. So isn't a public module.
// # Console Input/Output Module
// This module provides functions for interacting with the console, including input and output operations.
use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


/// This ask() function is still a prototype, so **it could not work as expected**.
/// 
/// Ask for input from the console
/// 
/// ### Parameters:
/// - `T: std::str::FromStr` - The type of the input
/// 
/// ### Returns:
/// - `T` - The input
pub fn ask<T: std::str::FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<T>().unwrap();
}

// console::clear_screen();
// console::set_color("red");
// console::print("Hello, world!");
// console::reset_color();
// console::print("Hello, world!");
// console::set_color("green");


/// Pause the program until the user presses enter.
/// 
/// This function will print a message to the console and wait for the user to press enter.
#[inline]
fn pause() {
    println!("Press enter to exit...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("Error reading line");
}
