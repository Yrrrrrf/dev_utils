//! A flexible and customizable logging module for Rust applications.
//!
//! This module provides a logging system with support for multiple log levels,
//! colored output, and customizable formatting. It's designed to be easy to use
//! while still offering advanced features for those who need them.
//!
//! # Features
//! - Five log levels: Trace, Debug, Info, Warn, and Error
//! - Colored output for easy visual distinction between log levels
//! - Customizable log formatting through the `DlogStyle` trait
//! - Atomic log level setting for thread-safe operation
//! - Macros for easy logging at different levels
//!
//! # Examples
//! ```
//! use dev_utils::dlog::*;
//!
//! // Set the maximum log level
//! set_max_level(Level::Debug);
//!
//! // Log messages at different levels
//! error!("This is an error message");
//! warn!("This is a warning message");
//! info!("This is an info message");
//! debug!("This is a debug message");
//! trace!("This is a trace message"); // This won't be printed due to log level
//! ```
use crate::format::{Color, Style, Stylize};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::{__dlog_internal, debug, error, info, trace, warn};

macro_rules! define_levels {
    ($($level:ident => $value:expr, $color:expr),+ $(,)?) => {
        /// Represents the log level of a message.
        ///
        /// The log levels are ordered from most detailed to least detailed.
        /// Each level has an associated color for use in the terminal.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum Level {
            $($level = $value),+
        }

        impl fmt::Display for Level {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Level::$level => write!(f, stringify!($level))),+
                }
            }
        }

        impl Level {
            fn color(&self) -> Color {
                match self {
                    $(Level::$level => $color),+
                }
            }
        }
    };
}

define_levels! {
    Trace => 5, Color::new(218,   0, 192),
    Debug => 4, Color::new( 96, 216, 216),
    Info  => 3, Color::new( 24, 216,  16),
    Warn  => 2, Color::new(232, 232,  64),
    Error => 1, Color::new(232,  72,  96),
}

static MAX_LOG_LEVEL: AtomicUsize = AtomicUsize::new(0);

/// Sets the maximum log level.
///
/// Only log messages with a severity level equal to or higher than this will be displayed.
///
/// # Arguments
///
/// * `level` - The maximum `Level` to log
///
/// # Examples
///
/// ```
/// use dev_utils::dlog::{set_max_level, Level};
///
/// set_max_level(Level::Info);  // Only log Info, Warn, and Error messages
/// ```
pub fn set_max_level(level: Level) {
    MAX_LOG_LEVEL.store(level as usize, Ordering::SeqCst);
}

/// Checks if a given log level is enabled.
///
/// # Arguments
///
/// * `level` - The `Level` to check
///
/// # Returns
///
/// `true` if the level is enabled, `false` otherwise
///
/// # Examples
///
/// ```
/// use dev_utils::dlog::{enabled, Level, set_max_level};
///
/// set_max_level(Level::Info);
/// assert!(enabled(Level::Error));
/// assert!(!enabled(Level::Debug));
/// ```
pub fn enabled(level: Level) -> bool {
    level as usize <= MAX_LOG_LEVEL.load(Ordering::Relaxed)
}

/// Removes ANSI escape sequences from a string.
///
/// This function is used internally to calculate the visual length of log messages.
///
/// # Arguments
///
/// * `src_str` - The input string that may contain ANSI escape sequences
///
/// # Returns
///
/// A new `String` with all ANSI escape sequences removed
fn strip_ansi_escapes(src_str: &str) -> String {
    let mut result = String::with_capacity(src_str.len());
    let mut in_escape = false;
    src_str.chars().for_each(|c| match c {
        '\x1B' => in_escape = true,
        'm' if in_escape => in_escape = false,
        _ if !in_escape => result.push(c),
        _ => (), // do nothing when the str is an escape sequence (e.g. \x1B[90m)
    });
    result
}

const LEVEL_WIDTH: usize = 0x05; // * Just an unsigned integer w/ a fancy declaration

/// Trait for customizing log message formatting.
pub trait DlogStyle {
    /// Formats a log message.
    ///
    /// This method can be overridden to customize the appearance of log messages.
    ///
    /// # Arguments
    ///
    /// * `level` - The `Level` of the log message
    /// * `args` - The message content as `fmt::Arguments`
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted log message
    fn format_log(&self, level: &Level, args: fmt::Arguments) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let secs = now.as_secs();
        let ms = now.subsec_millis();

        let (hr, min, sec) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);

        let timestamp = format!("\x1b[90m[{hr:02}:{min:02}:{sec:02}.{ms:03}]\x1b[0m");

        let level_str = level.to_string();
        let level_str = self.level_color(
            level,
            &format!(
                "{level_str:>width$}",
                width = LEVEL_WIDTH - ((LEVEL_WIDTH - level_str.len()) / 2)
            ),
        );

        let prefix = format!("{} {} ", timestamp, level_str);
        let content_start = strip_ansi_escapes(&prefix).len();

        let binding = args.to_string();
        let (lines, overall_style) = parse_styled_lines(&binding);
        let line_count = lines.len();

        let mut output = String::new();
        for (i, line) in lines.into_iter().enumerate() {
            let formatted_line = if i == 0 {
                format!("{}{}{}", prefix, overall_style, line)
            } else {
                let line_prefix = if i == line_count - 1 { "└" } else { "│" };
                format!(
                    "\n{}{} {}{}",
                    " ".repeat(content_start - 2),
                    self.level_color(level, line_prefix),
                    overall_style,
                    line
                )
            };
            output.push_str(&formatted_line);
        }
        // Add the reset code at the very end
        output.push_str("\x1b[0m");
        output
    }

    /// Applies color to the level indicator in the log message.
    ///
    /// # Arguments
    ///
    /// * `level` - The `Level` of the log message
    /// * `msg` - The level indicator string
    ///
    /// # Returns
    ///
    /// A `String` containing the colored level indicator
    fn level_color(&self, level: &Level, msg: &str) -> String {
        format!("{:?}{}\x1b[0m", level.color(), msg)
    }
}

/// Parses a string into lines, extracting any overall style.
///
/// This function is used internally to handle multi-line log messages and preserve styling.
///
/// # Arguments
///
/// * `input` - The input string to parse
///
/// # Returns
///
/// A tuple containing a `Vec<String>` of parsed lines and a `String` with any overall style
fn parse_styled_lines(input: &str) -> (Vec<String>, String) {
    let mut lines = Vec::new();
    let mut overall_style = String::new();

    for line in input.lines() {
        if line.starts_with("\x1b[") {
            let style_end = line.find('m').map(|i| i + 1).unwrap_or(0);
            overall_style = line[..style_end].to_string();
            lines.push(line[style_end..].to_string());
        } else {
            lines.push(line.to_string());
        }
    }

    (lines, overall_style)
}

/// The default implementation of `DlogStyle`.
pub struct DefaultDlogStyle;

impl DlogStyle for DefaultDlogStyle {
    fn level_color(&self, level: &Level, msg: &str) -> String {
        msg.color(level.color()).style(Style::Bold)
    }
}

/// Logs a message with the given style and level.
///
/// This function is the core of the logging system and is typically called through the logging macros.
///
/// # Arguments
///
/// * `style` - The `DlogStyle` to use for formatting
/// * `level` - The `Level` of the log message
/// * `args` - The message content as `fmt::Arguments`
pub fn log(style: &impl DlogStyle, level: Level, args: fmt::Arguments) {
    if enabled(level) {
        let log_message = style.format_log(&level, args);
        println!("{}", log_message);
    }
}

#[macro_export]
macro_rules! __dlog_internal {
    ($level:expr, $($arg:tt)+) => {
        $crate::dlog::log(&$crate::dlog::DefaultDlogStyle, $level, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! error { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Error, $($arg)+) }; }
#[macro_export]
macro_rules! warn  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Warn,  $($arg)+) }; }
#[macro_export]
macro_rules! info  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Info,  $($arg)+) }; }
#[macro_export]
macro_rules! debug { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Debug, $($arg)+) }; }
#[macro_export]
macro_rules! trace { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Trace, $($arg)+) }; }

// todo: Improve this code by implemeneting some PROC MACRO
// todo: that will generate the following macros.
// todo: Because the code below is repetitive, so it can be generated.
