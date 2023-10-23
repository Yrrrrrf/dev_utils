//! Terminal Utilities
//! 
//! These are utilities for the terminal.
//! It allow us to change the color of the text in the terminal.
//! 
//! Also have some functions to ask for input from the console.

// ? Command Line Interface (CLI) related scripts  -------------------------------------------------    -------------------------------

/// **Sets the foreground color** of a given string for ANSI terminal output.
///
/// This function takes an input string and a color specifier (abbreviated or full names)
/// and formats the string with ANSI escape codes to set the foreground color of the text
/// when printed to a terminal that supports ANSI color codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to colorize.
/// - `fg` - A color specifier that can be a single character or an abbreviation
///          (e.g., "r" for red, "g" for green, "blue" for blue).
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the foreground color.
///
/// # Examples
///
/// ```
/// use dev_utils::terminal::set_fg;
/// 
/// println!("{}", set_fg("Hello, Red!", "r"));
/// println!("{}", set_fg("World in Green!", "green"));
/// ```
///
/// # Color Specifiers
///
/// The following color specifiers are supported:
///
/// - `"r"` or `"red"` - Red
/// - `"g"` or `"green"` - Green
/// - `"b"` or `"blue"` - Blue
/// - `"c"` or `"cyan"` - Cyan
/// - `"m"` or `"magenta"` - Magenta
/// - `"y"` or `"yellow"` - Yellow
/// - `Anything else` - Default (reset color)
/// 
/// Any other color specifier or invalid input will reset the text color to default.
///
/// Note: This function is intended for use with ANSI terminal emulators that support
/// ANSI color codes. The behavior may vary on terminals that do not support ANSI codes.
pub fn set_fg(string: &str, fg: impl Into<String>) -> String {
    format!("\x1b[{}m{}\x1b[0m", match fg.into().as_str() {
        "r" | "red" => "31",       // Red
        "g" | "green" => "32",     // Green
        "b" | "blue" => "34",      // Blue
        "c" | "cyan" => "36",      // Cyan
        "m" | "magenta" => "35",   // Magenta
        "y" | "yellow" => "33",    // Yellow
        _ => "0",                  // Default (reset color)
    }, string)
}


/// **Sets the background color** of a given string for ANSI terminal output.
///
/// This function takes an input string and a color specifier (abbreviated or full names)
/// and formats the string with ANSI escape codes to set the background color of the text
/// when printed to a terminal that supports ANSI color codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to colorize.
/// - `bg` - A color specifier that can be a single character or an abbreviation
///          (e.g., "r" for red, "g" for green, "blue" for blue).
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the background color.
///
/// # Examples
///
/// ```
/// use dev_utils::terminal::set_bg;
///
/// println!("{}", set_bg("Hello, Red Background!", "r"));
/// println!("{}", set_bg("World in Green Background!", "green"));
/// ```
///
/// # Color Specifiers
///
/// The following color specifiers are supported:
///
/// - `"r"` or `"red"` - Red
/// - `"g"` or `"green"` - Green
/// - `"b"` or `"blue"` - Blue
/// - `"c"` or `"cyan"` - Cyan
/// - `"m"` or `"magenta"` - Magenta
/// - `"y"` or `"yellow"` - Yellow
///
/// Any other color specifier or invalid input will reset the background color to default.
///
/// Note: This function is intended for use with ANSI terminal emulators that support
/// ANSI color codes. The behavior may vary on terminals that do not support ANSI codes.
pub fn set_bg(string: &str, bg: impl Into<String>) -> String {
    format!("\x1b[48;5;{}m{}\x1b[0m", match bg.into().as_str() {
        "r" | "red" => "196",     // Red
        "g" | "green" => "46",   // Green
        "b" | "blue" => "21",     // Blue
        "c" | "cyan" => "51",     // Cyan
        "m" | "mage" => "201",    // Magenta
        "y" | "yellow" => "3", // Yellow
        "w" | "white" => "15",        // White
        "black" => "0",         // Black
        _ => "256"  // Transparent
    }, string)
}


// ? Ask for input  ---------------------------------------------------------------------------------------------------------------


use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


/// This ask() function is still a prototype, so **it could not work as expected**.

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


/// Print the type of a variable
/// 
/// ### Parameters:
/// - `_: &T` - The variable to print the type of
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


/// Return the type of a variable as a string
/// 
/// ### Parameters:
/// - `_: &T` - The variable to get the type of
/// 
/// ### Returns:
/// - [`String`] - The type of the variable
pub fn get_type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}
