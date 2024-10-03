use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::fmt;
use std::str::FromStr;
use std::error::Error;


macro_rules! define_date_time_structs {
    (
        $(
            $name:ident {  // Struct name
                $($field:ident: $type:ty),+ $(,)?  // field: type, ...
            }
        ),+ $(,)?  // Optional trailing comma, ...
    ) => {
        $(
            // PartialOrd impl the <, <=, >, >= operators
            // PartialEq impl the == and != operators
            // Eq impl the == operator
            // Ord impl the cmp() method (this means that the struct can be sorted)
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name {
                $(pub $field: $type),+
            }
        )+
    };
}

// Usage of the macro
define_date_time_structs! {
    Date { year: i32, month: u8, day: u8},
    Time { hour: u8, minute: u8, second: u8},
    DateTime { date: Date, time: Time }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    InvalidYear(i32),
    InvalidMonth(u8),
    InvalidDay(u8),
    InvalidHour(u8),
    InvalidMinute(u8),
    InvalidSecond(u8),
    InvalidDate { year: i32, month: u8, day: u8 },
    InvalidTime { hour: u8, minute: u8, second: u8 },
    ParseError(String),
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidYear(year) => write!(f, "Invalid year: {}", year),
            Self::InvalidMonth(month) => write!(f, "Invalid month: {}", month),
            Self::InvalidDay(day) => write!(f, "Invalid day: {}", day),
            Self::InvalidHour(hour) => write!(f, "Invalid hour: {}", hour),
            Self::InvalidMinute(minute) => write!(f, "Invalid minute: {}", minute),
            Self::InvalidSecond(second) => write!(f, "Invalid second: {}", second),
            Self::InvalidDate { year, month, day } => write!(f, "Invalid date: {}-{}-{}", year, month, day),
            Self::InvalidTime { hour, minute, second } => write!(f, "Invalid time: {}:{}:{}", hour, minute, second),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for DateTimeError {}


impl Date {
    pub const fn new(year: i32, month: u8, day: u8) -> Result<Self, DateTimeError> {
        match (month, day) {
            (m, d) if m >= 1 && m <= 12 && d >= 1 && d <= Self::days_in_month(year, m) => 
                Ok(Self { year, month: m, day: d }),
            (m, _) if m < 1 || m > 12 => Err(DateTimeError::InvalidMonth(m)),
            (_, d) => Err(DateTimeError::InvalidDay(d)),
        }
    }

    pub const fn days_in_month(year: i32, month: u8) -> u8 {
        const DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        DAYS[month as usize - 1] + ((month == 2 && Self::is_leap_year(year)) as u8)
    }

    pub const fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
}

impl Time {
    pub const fn new(hour: u8, minute: u8, second: u8) -> Result<Self, DateTimeError> {
        match (hour, minute, second) {
            (h, m, s) if h < 24 && m < 60 && s < 60 => Ok(Self { hour: h, minute: m, second: s }),
            (h, _, _) if h >= 24 => Err(DateTimeError::InvalidHour(h)),
            (_, m, _) if m >= 60 => Err(DateTimeError::InvalidMinute(m)),
            (_, _, s) if s >= 60 => Err(DateTimeError::InvalidSecond(s)),
            _ => unreachable!() // This case should never happen due to the nature of u8
        }
    }
}

impl DateTime {
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self::from_timestamp(now.as_secs() as i64).unwrap()
    }

    pub fn from_timestamp(timestamp: i64) -> Result<Self, DateTimeError> {
        let (days, seconds) = (timestamp / 86400, timestamp % 86400);
        let (year, month, day) = Self::calculate_ymd(days);
        let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);

        Ok(Self {
            date: Date::new(year, month as u8, (day + 1) as u8)?,
            time: Time::new(hour as u8, minute as u8, second as u8)?,
        })
    }

    // calculate year, month, and day from days since 1970-01-01
    fn calculate_ymd(mut days: i64) -> (i32, u8, u8) {
        let mut year = 1970;
        let mut month = 1;

        while days >= 365 + Date::is_leap_year(year) as i64 {
            days -= 365 + Date::is_leap_year(year) as i64;
            year += 1;
        }

        while days >= Date::days_in_month(year, month) as i64 {
            days -= Date::days_in_month(year, month) as i64;
            month += 1;
        }

        (year, month, days as u8 + 1)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",  // 2023-05-01 12:34:56
            self.date.year, self.date.month, self.date.day,  // date
            self.time.hour, self.time.minute, self.time.second  // time
        )
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(DateTimeError::ParseError("Invalid format".to_string()));
        }

        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();

        if date_parts.len() != 3 || time_parts.len() != 3 {
            return Err(DateTimeError::ParseError("Invalid format".to_string()));
        }

        let year = date_parts[0].parse().map_err(|_| DateTimeError::ParseError("Invalid year".to_string()))?;
        let month = date_parts[1].parse().map_err(|_| DateTimeError::ParseError("Invalid month".to_string()))?;
        let day = date_parts[2].parse().map_err(|_| DateTimeError::ParseError("Invalid day".to_string()))?;

        let hour = time_parts[0].parse().map_err(|_| DateTimeError::ParseError("Invalid hour".to_string()))?;
        let minute = time_parts[1].parse().map_err(|_| DateTimeError::ParseError("Invalid minute".to_string()))?;
        let second = time_parts[2].parse().map_err(|_| DateTimeError::ParseError("Invalid second".to_string()))?;

        Ok(Self {
            date: Date::new(year, month, day)?,
            time: Time::new(hour, minute, second)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_creation() {
        assert!(Date::new(2023, 4, 30).is_ok());
        assert!(Date::new(2023, 2, 29).is_err());
        assert!(Date::new(2024, 2, 29).is_ok());
    }

    #[test]
    fn test_time_creation() {
        assert!(Time::new(23, 59, 59).is_ok());
        assert!(Time::new(24, 0, 0).is_err());
    }

    #[test]
    fn test_datetime_from_timestamp() {
        let dt = DateTime::from_timestamp(1682899200).unwrap();
        assert_eq!(dt.to_string(), "2023-05-01 00:00:00");
    }

    #[test]
    fn test_datetime_parsing() {
        let dt: DateTime = "2023-05-01 12:34:56".parse().unwrap();
        assert_eq!(dt.to_string(), "2023-05-01 12:34:56");
    }

    #[test]
    fn test_error_display() {
        let err = DateTimeError::InvalidYear(2023);
        assert_eq!(err.to_string(), "Invalid year: 2023");
    }
}
