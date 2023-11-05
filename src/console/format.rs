//! Console formatting utilities.
//! 
//! 


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


pub fn set_fg_rgb(string: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, string)
}


pub fn set_bg_rgb(string: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, string)
}


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


fn set_fg_256(string: &str, color: u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", color, string)
}
