//! A Rust Logger Implementation
//!
//! RLog is a lightweight logger implementation for Rust that allows you to control
//! the log level and print log records to the terminal with colorful output.
//!

//! RLog provides the following features:
//!
//! - Control the log level to display only logs of certain severity.
//! - Colorful output for log records to make them easily distinguishable.
//! - Customizable timestamp format and log record formatting.
//!
//! # Log Levels
//!
//! RLog uses the following log levels:
//!
//! - `Trace`: The most detailed log level.
//! - `Debug`: Used for debugging information.
//! - `Info`: General information about the application.
//! - `Warn`: Indicates a potential issue or warning.
//! - `Error`: Represents errors in the application.
//!
//! # Example
//!
//! ```rust
//! use dev_utils::rlog::RLog;
//! use log::LevelFilter;
//!
//! fn main() {
//!     RLog::init_logger(LevelFilter::Info);
//!
//!     log::info!("This is an informational message.");
//!     log::error!("This is an error message.");
//! }
//! ```
//! 
//! # Note
//!
//! To use this logger, you need to include it in your dependencies and initialize it in your application.
//! Make sure to set the `RUST_LOG` environment variable to control the log level (e.g., `RUST_LOG=info`).
#![allow(unused)]

use log::{Log, Level, Metadata, Record, LevelFilter};
use crate::conversion::datetime::now;


/// The `RLog` struct represents a logger.
/// 
/// It is used for logging messages in Rust programs.
/// 
/// # Examples
/// ```rust
/// use dev_utils::rlog::RLog;
/// use log::LevelFilter;
/// 
/// RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level
/// log::info!("Some data!");  // [2021-01-01 00:00:00] INFO: Hello World!
/// log::warn!("Warn!");  // [2021-01-01 00:00:00] WARN: Hello World!
/// ```
pub struct RLog;

impl RLog {
    pub fn init_logger(level: LevelFilter) {
        log::set_logger(&RLog).unwrap();
        log::set_max_level(level);   // Set the max log level to use
    }
}

impl Log for RLog {
    /// Returns true if the given metadata's level is less than or equal to the log level.
    /// 
    /// # Arguments
    /// - `metadata` [Metadata] - The metadata to check.
    /// 
    /// # Returns
    /// - [bool] - True if the given metadata's level is less than or equal to the log level.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }


    /// Prints the given record to the terminal.
    /// 
    /// # Arguments
    /// - `record` [Record] - The record to print.
    fn log(&self, record: &Record) {
        let (year, month, day, hour, minute, second) = crate::conversion::datetime::now(-6);

        if self.enabled(record.metadata()) {
            println!("\x1b[90m[{:20}{:>12}]\x1b[0m {:<14} {}",
                // format!("{year:4}-{month:0>2}-{day:0>2} {hour:0>2}:{minute:0>2}:{second:0>2}"),

                // todo: Add this format to the datetime module (dev_utils)
                format!("{year:4}-{:0>3}-{day:0>2} {hour:0>2}:{minute:0>2}:{second:0>2}", 
                match month {
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    _ => "Err",
                }),
                record.target(),
                format!("\x1b[{}m{}\x1b[0m", match record.level() {
                    Level::Trace => "36",  // Cyan
                    Level::Debug => "34",  // Blue
                    Level::Info => "32",  // Green
                    Level::Warn => "33",  // Yellow
                    Level::Error => "31",  // Red
                    // _ => "0",  // Not really needed since the log level is already checked in the enabled method
                }, record.level()),
                record.args()
            );
        }
    }


    /// Flushes the logger (the log is written to the output)
    /// todo: implement the flush method
    fn flush(&self) {
        // Implement any necessary flushing logic if needed
        // (e.g., for buffered logging)
    }
}
