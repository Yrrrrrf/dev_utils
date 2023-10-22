/// The main functions are to print the app data and initialize the logger.

// todo: DECIDE WHAT TO DO WITH THIS FILE (and all inside the util directory)

// * The functions in this file are not an integral part of the program
// * They are just utilities that are used throughout the program
// * So, they can be moved to a separate crate (e.g., logic_tracer_util)
// Or maybe just make it private. This will affect the documentation
// (Doctest can't access private functions)


use log::{Log, Level, Metadata, Record, LevelFilter, warn, info, debug, trace, error};
use std::time::{Duration, UNIX_EPOCH, SystemTime};

use crate::terminal;

/// Reads the Cargo.toml file and prints the app data
/// 
/// This is also the same as the package data.
pub fn print_app_data() {
    // print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal
    print!("{}", terminal::set_fg(&env!("CARGO_PKG_NAME").to_uppercase(), "g"));
    println!(" {}", terminal::set_fg(&format!("V{}", env!("CARGO_PKG_VERSION")), "b"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!();
}


/// The `RLog` struct represents a logger.
/// 
/// It is used for logging messages in Rust programs.
/// 
/// # Examples
/// ```rust
/// use logic_tracer::util::RLog;
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
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
        timestamp -= 6 * 3600;  // remove 6 hours from the timestamp
        let (days, hours, minutes, seconds) = calculate_hour_minute_second(timestamp);
        let (years, months, days) = calculate_year_month_day(days);

        if self.enabled(record.metadata()) {
            println!("\x1b[90m[{} {:>16}]\x1b[0m {:16} {}",
                format!("{:4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", 
                    years, months, days, hours, minutes, seconds),
                record.target(),
                format!("\x1b[{}m{}\x1b[0m", match record.level() {
                    // CYAN TRACE
                    Level::Trace => "36",  // Cyan
                    Level::Debug => "34",  // Blue
                    Level::Info => "32",  // Green
                    Level::Warn => "33",  // Yellow
                    Level::Error => "31",  // Red
                    // _ => "0",
                }, record.level()),
                record.args()
            );
        }
    }


    /// Flushes the logger.
    /// flushed: neans that the log is written to the output
    /// todo: implement the flush method
    fn flush(&self) {
        // Implement any necessary flushing logic if needed
        // (e.g., for buffered logging)
    }
}


// ? Time Utils -------------------------------------------------------------------------------------------------------------------


/// Calculates the days, hours, minutes, and seconds from the given timestamp.
/// 
/// # Arguments
/// - `timestamp` [u64] - The time in seconds since the UNIX epoch.
/// 
/// # Returns
/// - [u64] - The number of complete days.
/// - [u64] - The number of hours.
/// - [u64] - The number of minutes.
/// - [u64] - The number of seconds.
/// 
/// # Examples
/// ```rust
/// use logic_tracer::util::calculate_hour_minute_second; // Update the import path
/// 
/// assert_eq!(calculate_hour_minute_second(1), (0, 0, 0, 1));  // 1 second
/// assert_eq!(calculate_hour_minute_second(60), (0, 0, 1, 0));  // 1 minute
/// assert_eq!(calculate_hour_minute_second(3600), (0, 1, 0, 0));  // 1 hour
/// assert_eq!(calculate_hour_minute_second(86400), (1, 0, 0, 0));  // 1 day
fn calculate_hour_minute_second(timestamp: u64) -> (u64, u64, u64, u64) {
    const SECONDS_IN_A_DAY: u64 = 24 * 60 * 60;
    let days = timestamp / SECONDS_IN_A_DAY;
    let remaining_seconds = timestamp % SECONDS_IN_A_DAY;
    let hours = remaining_seconds / 3600;
    let remaining_seconds = remaining_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    (days, hours, minutes, seconds)
}


/// Calculates the year, month, and day from the given number of days.
/// 
/// # Arguments
/// - `days` [u64] - The number of days to calculate the year, month, and day from.
/// 
/// # Returns
/// - [u64] - The year.
/// - [u8] - The month.
/// - [u64] - The day.
/// 
/// # Examples
/// ```
/// use logic_tracer::util::calculate_year_month_day; // Update the import path
/// 
/// assert_eq!(calculate_year_month_day(1), (1970, 1, 2));  // 1 day
/// assert_eq!(calculate_year_month_day(365), (1971, 1, 1));  // 1 year
/// assert_eq!(calculate_year_month_day(366), (1971, 1, 2));  // 1 year and 1 day
/// ```
pub fn calculate_year_month_day(mut days: u64) -> (u64, u8, u64) {
    let mut year = 1970;
    let mut month = 1;
    let mut day = 1;
    while days >= days_in_year(year) {
        days -= days_in_year(year);
        year += 1;
    }
    while days >= days_in_month(year, month) {
        days -= days_in_month(year, month);
        month += 1;
    }
    day += days;
    (year, month, day)
}


/// Returns the number of days in the given year.
/// 
/// # Arguments
/// - `year` [u64] - The year to check.
/// 
/// # Returns
/// - [u64] - The number of days in the given year.
/// 
/// # Examples
/// ```rust
/// use logic_tracer::util::days_in_year; // Update the import path
/// 
/// assert_eq!(days_in_year(1970), 365);
/// assert_eq!(days_in_year(1971), 365);
/// assert_eq!(days_in_year(1972), 366);
/// assert_eq!(days_in_year(1975), 365);
/// assert_eq!(days_in_year(1976), 366);
/// ```
pub fn days_in_year(year: u64) -> u64 {
    if is_leap_year(year) { 366 } else { 365 }
}


/// Returns the number of days in the given month of the given year.
/// 
/// # Arguments
/// - `year` [u64] - The year to check.
/// - `month` [u8] - The month to check.
/// 
/// # Returns
/// - [u64] - The number of days in the given month of the given year. 0 if the month is invalid.
/// 
/// # Examples
/// ```rust
/// use logic_tracer::util::days_in_month;
/// 
/// assert_eq!(days_in_month(1970, 1), 31);
/// assert_eq!(days_in_month(1970, 2), 28);
/// assert_eq!(days_in_month(1970, 12), 31);
// assert_eq!(days_in_month(1970, 13), 0);  // invalid month
/// ```
pub fn days_in_month(year: u64, month: u8) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {if is_leap_year(year) {29} else {28}}
        _ => 0,
    }
}


/// Returns true if the given year is a leap year.
/// 
/// # Arguments
/// - `year` [u64] - The year to check.
/// 
/// # Returns
/// - [bool] - True if the given year is a leap year.
/// 
/// # Examples
/// ```rust
/// use logic_tracer::util::is_leap_year; // Update the import path
/// 
/// assert_eq!(is_leap_year(1970), false);
/// assert_eq!(is_leap_year(1971), false);
/// assert_eq!(is_leap_year(1972), true);
/// assert_eq!(is_leap_year(1975), false);
/// assert_eq!(is_leap_year(1976), true);
/// ```
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
