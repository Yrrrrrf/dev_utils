//! A flexible and customizable logging module for Rust applications.
//!
//! This module provides a logging system with support for multiple log levels,
//! colored output, and customizable formatting. It's designed to be easy to use
//! while still offering advanced features for those who need them.
//!
//! # Features
//! - Five log levels: Trace, Debug, Info, Warn, and Error
//! - Colored output for easy visual distinction between log levels
//! - Customizable log formatting through the `DlogStyle` trait (though only `DefaultDlogStyle` is provided)
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
use crate::format::{Color, Style, Stylize}; // Assuming these are available from your `format` module
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Re-export logging macros for convenient use (e.g., `dev_utils::info!`)
// Also re-exports the internal macro helper.
pub use crate::{__dlog_internal, debug, error, info, trace, warn};

/// Defines the `Level` enum and its associated methods.
/// This macro reduces boilerplate for defining log levels, their order,
/// display format, and associated colors.
macro_rules! define_levels {
    ($($level:ident => $value:expr, $color:expr),+ $(,)?) => {
        /// Represents the log level of a message.
        ///
        /// The log levels are ordered from most detailed (Trace) to least detailed (Error).
        /// Each level has an associated color for use in the terminal.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum Level {
            /// The "trace" level. Designates very low priority, often extremely verbose, information.
            Trace = 5, // Explicitly define for clarity, though macro could infer
            /// The "debug" level. Designates lower priority information.
            Debug = 4,
            /// The "info" level. Designates useful information.
            Info = 3,
            /// The "warn" level. Designates hazardous situations.
            Warn = 2,
            /// The "error" level. Designates very serious errors.
            Error = 1,
        }

        impl fmt::Display for Level {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    // Generate string representation for each level
                    $(Level::$level => write!(f, stringify!($level))),+
                }
            }
        }

        impl Level {
            /// Returns the `Color` associated with this log level.
            /// Used for styling the log output.
            fn color(&self) -> Color {
                match self {
                    // Assign a color to each level
                    $(Level::$level => $color),+
                }
            }
        }
    };
}

// Define the standard log levels using the macro.
// Values determine ordering (higher value = more verbose, lower numeric value = higher priority).
define_levels! {
    Trace => 5, Color::new(218,   0, 192), // Magenta-ish for Trace
    Debug => 4, Color::new( 96, 216, 216), // Cyan-ish for Debug
    Info  => 3, Color::new( 24, 216,  16), // Green-ish for Info
    Warn  => 2, Color::new(232, 232,  64), // Yellow-ish for Warn
    Error => 1, Color::new(232,  72,  96), // Red-ish for Error
}

/// Atomically stores the current maximum log level.
/// Log messages with a level numerically higher than this value will be ignored.
/// Defaults to `Level::Info`.
static MAX_LOG_LEVEL: AtomicUsize = AtomicUsize::new(Level::Info as usize);

/// Sets the global maximum log level.
///
/// Only log messages with a severity level equal to or numerically lower than
/// this value will be displayed (e.g., if `Level::Info` is set, `Info`, `Warn`,
/// and `Error` messages will be shown).
///
/// # Arguments
/// * `level` - The maximum `Level` to log.
///
/// # Examples
/// ```
/// use dev_utils::dlog::{set_max_level, Level};
/// set_max_level(Level::Warn); // Only Warn and Error messages will be shown.
/// ```
pub fn set_max_level(level: Level) {
    MAX_LOG_LEVEL.store(level as usize, Ordering::SeqCst);
}

/// Checks if a given log level is currently enabled based on `MAX_LOG_LEVEL`.
///
/// # Arguments
/// * `level` - The `Level` to check.
///
/// # Returns
/// `true` if messages at the given `level` should be logged, `false` otherwise.
///
/// # Examples
/// ```
/// use dev_utils::dlog::{enabled, Level, set_max_level};
/// set_max_level(Level::Info);
/// assert!(enabled(Level::Error));
/// assert!(!enabled(Level::Debug));
/// ```
pub fn enabled(level: Level) -> bool {
    // Log levels are ordered such that higher numeric value means more verbose/lower priority.
    // So, a message is enabled if its level value is less than or equal to the max_log_level value.
    level as usize <= MAX_LOG_LEVEL.load(Ordering::Relaxed)
}

/// Removes ANSI escape sequences from a string.
///
/// This is a simplified internal helper used, for example, to calculate the visual
/// length of styled prefixes to ensure correct alignment of multi-line log messages.
///
/// # Arguments
/// * `src_str` - The input string that may contain ANSI escape codes.
///
/// # Returns
/// A new `String` with all ANSI escape sequences removed.
fn strip_ansi_escapes(src_str: &str) -> String {
    let mut result = String::with_capacity(src_str.len());
    let mut in_escape = false;
    for c in src_str.chars() {
        match c {
            '\x1B' => in_escape = true,
            'm' if in_escape => in_escape = false,
            _ if !in_escape => result.push(c),
            _ => (), // Character is part of an escape sequence, ignore it.
        }
    }
    result
}

/// Consistent visual width for level strings (e.g., "ERROR", " WARN", " INFO", "DEBUG", "TRACE").
/// Used for aligning log messages.
const LEVEL_WIDTH: usize = 5;

/// A trait for defining how log messages are formatted.
///
/// This allows for custom log appearances, although `dev_utils` currently
/// only provides `DefaultDlogStyle`.
pub trait DlogStyle {
    /// Formats a log record into a string ready for output.
    ///
    /// # Arguments
    /// * `level` - The `Level` of the log message.
    /// * `args` - The formatted message arguments, as produced by `format_args!`.
    ///
    /// # Returns
    /// A `String` containing the fully formatted log message.
    fn format_log(&self, level: &Level, args: fmt::Arguments) -> String;

    /// Applies color and style to the textual representation of a log level.
    ///
    /// This method can be overridden by `DlogStyle` implementors to customize
    /// how the level indicator (e.g., "INFO", "ERROR") is styled.
    /// The default implementation bolds the message and applies the level's
    /// predefined color.
    ///
    /// # Arguments
    /// * `level` - The `Level` being formatted.
    /// * `msg` - The string representation of the level (e.g., "INFO").
    ///
    /// # Returns
    /// A `String` containing the styled level indicator.
    fn level_color(&self, level: &Level, msg: &str) -> String {
        msg.color(level.color()).style(Style::Bold)
    }
}

/// The default style for formatting log messages.
///
/// It produces logs with a timestamp, level indicator, and the message.
/// Multi-line messages are indented appropriately.
pub struct DefaultDlogStyle;

impl DlogStyle for DefaultDlogStyle {
    /// Formats a log record using the default style.
    ///
    /// The format includes:
    /// - A timestamp (e.g., `[HH:MM:SS.mmm]`) styled with `Style::Dim`.
    /// - A level indicator (e.g., `INFO `, `ERROR`) styled with the level's color and bold.
    /// - The log message.
    ///
    /// For multi-line messages, subsequent lines are indented and prefixed with
    /// a continuation character (`│` or `└`), also styled with the level's color.
    /// Any ANSI styling applied by the user within the log message arguments is preserved.
    fn format_log(&self, level: &Level, args: fmt::Arguments) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = now.as_secs();
        let ms = now.subsec_millis();
        let (hr, min, sec) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);

        // Styled timestamp
        let timestamp_str = format!("[{hr:02}:{min:02}:{sec:02}.{ms:03}]");
        let styled_timestamp = timestamp_str.style(Style::Dim);

        // Level string, right-aligned and padded
        let level_display_str = level.to_string();
        let padded_level_str = format!("{level_display_str:>LEVEL_WIDTH$}"); // Right align
        let styled_level_indicator = self.level_color(level, &padded_level_str);

        // Prefix for the first line of the log message
        let first_line_prefix_styled = format!("{} {} ", styled_timestamp, styled_level_indicator);

        // Calculate the visual column where the message content starts, for indenting subsequent lines
        let content_start_column = strip_ansi_escapes(&first_line_prefix_styled).len();

        let user_message_str = args.to_string();
        let user_message_lines: Vec<&str> = user_message_str.lines().collect();

        let mut output = String::new();

        if user_message_lines.is_empty() {
            // Handle case where the log message itself is empty (e.g., `info!("")`)
            output.push_str(&first_line_prefix_styled);
        } else {
            // Append the styled prefix for the first line
            output.push_str(&first_line_prefix_styled);
            // Append the first line of the user's message (maintaining user's styling)
            output.push_str(user_message_lines[0]);

            // Handle subsequent lines of a multi-line message
            for (i, line_content) in user_message_lines.iter().enumerate().skip(1) {
                output.push('\n');
                // Indent subsequent lines to align with the content of the first line
                output.push_str(&" ".repeat(content_start_column.saturating_sub(2))); // Subtract 2 for "│ " or "└ "

                // Choose continuation character
                let continuation_char = if i == user_message_lines.len() - 1 {
                    "└ " // Last line of a multi-line message
                } else {
                    "│ " // Middle line of a multi-line message
                };
                // Style the continuation character and append it
                output.push_str(&self.level_color(level, continuation_char));
                // Append the content of the subsequent line (maintaining user's styling)
                output.push_str(line_content);
            }
        }

        // Append ANSI reset code at the very end of the entire log entry
        output.push_str("\x1b[0m");
        output
    }

    /// Styles the log level string (e.g., "INFO", "ERROR") for the default style.
    /// This implementation makes the level bold and applies its predefined color.
    fn level_color(&self, level: &Level, msg: &str) -> String {
        msg.color(level.color()).style(Style::Bold)
    }
}


/// Logs a message if its level is enabled.
///
/// This function is the core of the logging system. It formats the message
/// using the provided `style` and prints it to the console.
///
/// # Arguments
/// * `style` - An implementor of `DlogStyle` to use for formatting.
/// * `level` - The `Level` of the log message.
/// * `args` - The message content as `fmt::Arguments`, typically from `format_args!`.
pub fn log(style: &impl DlogStyle, level: Level, args: fmt::Arguments) {
    if enabled(level) {
        let log_message = style.format_log(&level, args);
        println!("{}", log_message);
    }
}

/// Internal macro used by the public logging macros (`error!`, `info!`, etc.).
/// It passes the log call to the `log` function with `DefaultDlogStyle`.
#[macro_export]
#[doc(hidden)] // Hide from public documentation as it's an internal detail.
macro_rules! __dlog_internal {
    ($level:expr, $($arg:tt)+) => {
        // Always use DefaultDlogStyle for messages logged via these macros.
        $crate::dlog::log(&$crate::dlog::DefaultDlogStyle, $level, format_args!($($arg)+))
    };
}

/// Logs a message at the Error level.
///
/// # Examples
/// ```
/// use dev_utils::error; // Assuming dlog macros are re-exported or dlog is in scope
/// error!("A critical error occurred: {}", "file not found");
/// ```
#[macro_export]
macro_rules! error { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Error, $($arg)+) }; }

/// Logs a message at the Warn level.
///
/// # Examples
/// ```
/// use dev_utils::warn;
/// warn!("A potential issue was detected: {}", "disk space low");
/// ```
#[macro_export]
macro_rules! warn  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Warn,  $($arg)+) }; }

/// Logs a message at the Info level.
///
/// # Examples
/// ```
/// use dev_utils::info;
/// info!("Application started successfully on port {}.", 8080);
/// ```
#[macro_export]
macro_rules! info  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Info,  $($arg)+) }; }

/// Logs a message at the Debug level.
///
/// Debug messages are typically stripped out in release builds unless explicitly enabled.
/// # Examples
/// ```
/// use dev_utils::debug;
/// let user_id = 123;
/// debug!("Processing request for user_id: {}", user_id);
/// ```
#[macro_export]
macro_rules! debug { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Debug, $($arg)+) }; }

/// Logs a message at the Trace level.
///
/// Trace messages are the most verbose and are usually stripped out in release builds.
/// # Examples
/// ```
/// use dev_utils::trace;
/// trace!("Entering function: process_data with value: {}", 42);
/// ```
#[macro_export]
macro_rules! trace { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Trace, $($arg)+) }; }

// todo: Improve this code by implemeneting some PROC MACRO
// todo: that will generate the following macros.
// todo: Because the code below is repetitive, so it can be generated.
