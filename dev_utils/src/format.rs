//! A module for text formatting and styling in terminal applications.
//!
//! This module provides functionality for adding colors and styles to text,
//! as well as utilities for working with ANSI escape codes. It includes a [Color] struct
//! for RGB color representation, a [Style] enum for text styles, and a [Stylize] trait
//! for applying these formats to strings.
//!
//! # Features
//! - RGB color support for both foreground and background
//! - Text styling (bold, italic, underline, etc.)
//! - ANSI escape code handling
//! - Utilities for stripping ANSI codes and calculating visual string length
//!
//! # Examples
//! ```
//! use dev_utils::format::{Color, Style, Stylize};
//! use dev_utils::format::{RED, WHITE};
//!
//! let text = "Hello, World!";
//! println!("{}", text.color(RED).on_color(WHITE).style(Style::Bold));
//! ```
use std::fmt;

/// Represents an RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates a new `Color` instance with the given RGB values.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Returns the RGB components as a tuple.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Returns the ANSI escape code for setting this color as the foreground color.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_utils::format::Color;
    ///
    /// let red = Color::new(255, 0, 0);
    /// println!("{}Red text\x1b[0m", red.as_fg());
    /// ```
    pub fn as_fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Returns the ANSI escape code for setting this color as the background color.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_utils::format::Color;
    ///
    /// let blue = Color::new(0, 0, 255);
    /// println!("{}Text with blue background\x1b[0m", blue.as_bg());
    /// ```
    pub fn as_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Creates a `Color` from an RGB tuple.
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color::new(rgb.0, rgb.1, rgb.2)
    }
}

// The `define_colors!` macro creates constant Color instances.
// Documentation for each color constant will be generated automatically.
macro_rules! define_colors {
    ($($name:ident => ($r:expr, $g:expr, $b:expr)),* $(,)?) => {
        $(pub const $name: Color = Color { r: $r, g: $g, b: $b };)*
    };
}

define_colors!(
    BLACK => (0, 0, 0),
    BLUE => (0, 0, 255),
    GREEN => (0, 255, 0),
    CYAN => (0, 255, 255),
    RED => (255, 0, 0),
    MAGENTA => (255, 0, 255),
    YELLOW => (255, 255, 0),
    WHITE => (255, 255, 255)
    // * define any custom colors here...
);

// Macro to create Style enum and implement style codes
macro_rules! create_style_enum {
    ($(($style:ident, $code:expr)),* $(,)?) => {
        /// Variants are created by the `create_style_enum!` macro.
        ///
        ///  Documentation for each style will be generated automatically.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Style {$($style,)*}

        impl Style {
            pub fn code(&self) -> String {
                match self {$(Style::$style => format!("\x1b[{}m", $code),)*}
            }
        }
    };
}

// Create Style enum and implement style codes
create_style_enum! {
    (Bold, 1),  // 1
    (Dim, 2),  // 1
    (Italic, 3),  // 1
    (Underline, 4),  // 1
    // (Blink, 5),  // 0 (does not work in most terminals)
    // (Reverse, 7),  // 0 (does not work in most terminals)
    (Hidden, 8),  // 1
}

/// A trait for applying colors and styles to text.
pub trait Stylize {
    /// Applies a color to the text.
    fn color(&self, color: Color) -> String;
    /// Applies a background color to the text.
    fn on_color(&self, color: Color) -> String;
    /// Applies a style to the text.
    fn style(&self, style: Style) -> String;
}

// Macro to implement Stylize for both &str and String
macro_rules! impl_stylize {
    ($($t:ty)*) => ($(
        impl Stylize for $t {
            fn color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.as_fg(), self)}
            fn on_color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.as_bg(), self)}
            fn style(&self, style: Style) -> String {format!("{}{}\x1b[0m", style.code(), self)}
        }
    )*)
}

// The `impl_stylize!` macro implements the Stylize trait for str and String.
impl_stylize! { str String }

/// Removes ANSI escape codes from a string.
///
/// This function uses a finite state machine to identify and remove ANSI escape sequences,
/// leaving only the visible text content.
///
/// # Arguments
///
/// * `s` - The input string that may contain ANSI escape codes
///
/// # Returns
///
/// A new `String` with all ANSI escape codes removed.
///
/// # Examples
///
/// ```
/// use dev_utils::format::strip_ansi_codes;
///
/// let colored_text = "\x1b[31mRed text\x1b[0m";
/// assert_eq!(strip_ansi_codes(colored_text), "Red text");
/// ```
pub fn strip_ansi_codes(s: &str) -> String {
    #[derive(Clone, Copy)]
    enum State {
        Normal,
        Escape,
        Csi,
    }

    // THIS FSM is used to strip ANSI escape codes from the given string
    // It scans through the characters and removes the ANSI codes
    s.chars()
        .scan(State::Normal, |state, c| match (*state, c) {
            (State::Normal, '\x1B') => {
                *state = State::Escape;
                Some(None)
            }
            (State::Escape, '[') => {
                *state = State::Csi;
                Some(None)
            }
            (State::Escape, _) => {
                *state = State::Normal;
                Some(Some('\x1B'))
            }
            (State::Csi, 'm') => {
                *state = State::Normal;
                Some(None)
            }
            (State::Csi, '0'..='9') | (State::Csi, ';') => Some(None),
            (State::Csi, _) => {
                *state = State::Normal;
                Some(Some(c))
            }
            (State::Normal, c) => Some(Some(c)),
        })
        .flatten()
        .collect()
}

/// Calculates the visual length of a string, ignoring ANSI escape codes.
///
/// This function first strips all ANSI escape codes from the input string and then
/// counts the remaining characters to determine the visual length.
///
/// # Arguments
///
/// * `s` - The input string that may contain ANSI escape codes
///
/// # Returns
///
/// The number of visible characters in the string.
///
/// # Examples
///
/// ```
/// use dev_utils::format::visual_length;
///
/// let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m";
/// assert_eq!(visual_length(colored_text), 9); // "Red Green"
/// ```
pub fn visual_length(s: &str) -> usize {
    strip_ansi_codes(s).chars().count()
}
