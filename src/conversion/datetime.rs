//! # Date and Time Utilities
//!
//! This module provides functions for working with dates and times in Rust, allowing you
//! to perform various calculations and checks related to timestamps, days, months, and years.
//! 
//! The methods in this module are useful for working with timestamps, dates, and times in Rust.
//! Trying to simplify the process of working with dates and times in Rust, making it easy
//!
//! The data types used on each method try to be as generic and smaller as possible to use less memory.
//! 
//! ## Contents
//!
//! - [`calculate_hour_minute_second`](fn.calculate_hour_minute_second.html): Calculate days,
//!   hours, minutes, and seconds from a given timestamp.
//! - [`calculate_year_month_day`](fn.calculate_year_month_day.html): Calculate the year, month,
//!   and day from a given number of days.
//! - [`days_in_year`](fn.days_in_year.html): Get the number of days in a specific year.
//! - [`days_in_month`](fn.days_in_month.html): Get the number of days in a specific month.
//! - [`is_leap_year`](fn.is_leap_year.html): Check if a given year is a leap year.
//!
//! ## Examples
//!
//! ```rust
//! use dev_utils::datetime::calculate_hour_minute_second;
//!
//! let timestamp = 3661; // 1 hour, 1 minute, and 1 second
//! let (days, hours, minutes, seconds) = calculate_hour_minute_second(timestamp);
//! assert_eq!(days, 0);
//! assert_eq!(hours, 1);
//! assert_eq!(minutes, 1);
//! assert_eq!(seconds, 1);
//! ```
//!
//! ```rust
//! use dev_utils::datetime::calculate_year_month_day;
//!
//! let days = 731; // 2 years and 1 day
//! let (year, month, day) = calculate_year_month_day(days);
//! assert_eq!(year, 1972);
//! assert_eq!(month, 1);
//! assert_eq!(day, 2);
//! ```
//!
//! ```rust
//! use dev_utils::datetime::is_leap_year;
//!
//! let year = 2024; // Leap year
//! let leap_year = is_leap_year(year);
//! assert_eq!(leap_year, true);
//! ```
//!
//! ## Notes
//!
//! - Leap year calculations are based on the Gregorian calendar.
//! It means that a leap year is every 4 years, except for years that are divisible by 100 and not divisible by 400.


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
/// use dev_utils::datetime::calculate_hour_minute_second; // Update the import path
/// 
/// assert_eq!(calculate_hour_minute_second(1), (0, 0, 0, 1));  // 1 second
/// assert_eq!(calculate_hour_minute_second(60), (0, 0, 1, 0));  // 1 minute
/// assert_eq!(calculate_hour_minute_second(3600), (0, 1, 0, 0));  // 1 hour
/// assert_eq!(calculate_hour_minute_second(86400), (1, 0, 0, 0));  // 1 day
pub fn calculate_hour_minute_second(timestamp: u64) -> (u64, u64, u64, u64) {
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
/// use dev_utils::datetime::calculate_year_month_day; // Update the import path
/// 
/// assert_eq!(calculate_year_month_day(1), (1970, 1, 2));  // 1 day
/// assert_eq!(calculate_year_month_day(365), (1971, 1, 1));  // 1 year
/// assert_eq!(calculate_year_month_day(366), (1971, 1, 2));  // 1 year and 1 day
/// ```
pub fn calculate_year_month_day(mut days: u64) -> (u64, u8, u64) {
    let mut year = 1970;
    let mut month = 1;
    let mut day = 1;
    while days >= days_in_year(year) as u64 {
        days -= days_in_year(year) as u64;
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
/// - [u16] - The number of days in the given year.
/// 
/// # Examples
/// ```rust
/// use dev_utils::datetime::days_in_year; // Update the import path
/// 
/// assert_eq!(days_in_year(1970), 365);
/// assert_eq!(days_in_year(1971), 365);
/// assert_eq!(days_in_year(1972), 366);
/// assert_eq!(days_in_year(1975), 365);
/// assert_eq!(days_in_year(1976), 366);
/// ```
pub fn days_in_year(year: u64) -> u16 {
    match is_leap_year(year) {
        true => 366,
        false => 365,
    }
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
/// use dev_utils::datetime::days_in_month;
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
/// use dev_utils::datetime::is_leap_year; // Update the import path
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
