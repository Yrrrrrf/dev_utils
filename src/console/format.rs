//! # Console Formatting Utilities
//!
//! This module provides utility functions for formatting text output to the console
//! with ANSI escape codes. These functions allow you to set text colors, background
//! colors, styles, and more, enhancing the visual appearance of your console output.
//!
//! # Example
//!
//! ```rust
//! use dev_utils::console::format::{set_fg, set_bg, set_style};
//!
//! // Print red text
//! println!("{}", set_fg("Hello, Red!", "red"));
//!
//! // Print text with green background
//! println!("{}", set_bg("World in Green Background!", "green"));
//!
//! // Print bold and italic text
//! println!("{}", set_style("Formatted Text", "bold"));
//! println!("{}", set_style("Italic Text", "italic"));
//! ```
//!
//! # Note
//!
//! These functions are intended for use with ANSI terminal emulators that support
//! ANSI color codes and styles. The behavior may vary on terminals that do not
//! support ANSI codes.


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
/// use dev_utils::console::format::set_fg;
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
/// use dev_utils::console::format::set_bg;
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


/// Sets the foreground color of a given string for ANSI terminal output using RGB values.
///
/// This function takes an input string and RGB color values (0-255) and formats the string
/// with ANSI escape codes to set the foreground color of the text when printed to a terminal
/// that supports ANSI color codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to colorize.
/// - `r` - Red component (0-255).
/// - `g` - Green component (0-255).
/// - `b` - Blue component (0-255).
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the foreground color.
///
/// # Examples
///
/// ```
///  use dev_utils::console::format::set_fg_rgb;
///
/// println!("{}", set_fg_rgb("Hello, Custom Color!", 255, 0, 0));  // Red text
/// ```
pub fn set_fg_rgb(string: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, string)
}


/// Sets the background color of a given string for ANSI terminal output using RGB values.
///
/// This function takes an input string and RGB color values (0-255) and formats the string
/// with ANSI escape codes to set the background color of the text when printed to a terminal
/// that supports ANSI color codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to colorize.
/// - `r` - Red component (0-255).
/// - `g` - Green component (0-255).
/// - `b` - Blue component (0-255).
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the background color.
///
/// # Examples
///
/// ```
///  use dev_utils::console::format::set_bg_rgb;
///
/// println!("{}", set_bg_rgb("Hello, Custom Background Color!", 0, 128, 255));  // Blue background
/// ```
pub fn set_bg_rgb(string: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, string)
}


/// Sets text styles such as bold, italic, underline, etc., for a given string.
///
/// This function takes an input string and a style specifier and formats the string
/// with ANSI escape codes to set various text styles when printed to a terminal
/// that supports ANSI style codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to stylize.
/// - `style` - A style specifier (e.g., "bold", "italic", "underline").
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting text styles.
///
/// # Examples
///
/// ```
///  use dev_utils::console::format::set_style;
///
/// println!("{}", set_style("Hello, Styled Text!", "bold"));
/// println!("{}", set_style("World in Italics!", "italic"));
/// ```
pub fn set_style(string: &str, style: impl Into<String>) -> String {
    format!("\x1b[{}m{}\x1b[0m", match style.into().as_str() {
        "bold" => "1",
        "dim" => "2",
        "italic" => "3",
        "underline" => "4",
        "blink" => "5",
        "reverse" => "7",
        "hidden" => "8",
        _ => "0"
    }, string)
}


/// Sets the foreground color of a given string for ANSI terminal output using a 256-color palette.
///
/// This function takes an input string and a color index (0-255) and formats the string
/// with ANSI escape codes to set the foreground color of the text when printed to a terminal
/// that supports ANSI color codes.
///
/// # Arguments
///
/// - `string` - The input string that you want to colorize.
/// - `color` - The color index in the 256-color palette (0-255).
///
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the foreground color.
///
/// # Examples
///
/// ```
///  use dev_utils::console::format::set_fg_256;
///
/// println!("{}", set_fg_256("Hello, Custom 256-color!", 123));  // Color index 123
/// ```
pub fn set_fg_256(string: &str, color: u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", color, string)
}
