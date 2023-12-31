//! This file contains the struct for a HTTP response.
//! 
//! # Example
//! ```
//! HTTP/1.1 / 200 OK
//! Date: Mon, 27 Jul 2009 12:28:53 GMT
//! Server: Apache/2.2.14 (Win32)
//! Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
//! Content-Length: 88
//! Content-Type: text/html
//! Connection: Closed
//! ```
use std::time::{SystemTime, UNIX_EPOCH};

use crate::conversion::datetime::{
    calculate_hour_minute_second, 
    calculate_year_month_day
};

use super::HttpVersion;
use super::HttpStatus;


#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub http_version: HttpVersion,
    // todo: Add the headers to the response
    // pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: HttpStatus, http_version: HttpVersion, body: impl Into<String>) -> HttpResponse {
        HttpResponse {status, http_version, body: body.into()}
    }

    pub fn new_1_1(status: HttpStatus, body: impl Into<String>) -> HttpResponse {
        HttpResponse {status, http_version: HttpVersion::Http1_1, body: body.into()}
    }
        

    /// Returns the current date and time in the format: 2021-08-01 16:00:00
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use http_response::HttpResponse;
    /// 
    /// let response = HttpResponse::new_1_1(HttpStatus::Ok, "Hello World!".to_string());
    /// println!("{}", response.to_string());
    /// ```
    pub fn now_hour_minute_second() -> String {
        // todo: Improve or create the now() fn in the datetime module (dev_utils)
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
        timestamp -= 6 * 3600;  // remove 6 hours from the timestamp
        let (days, hours, minutes, seconds) = calculate_hour_minute_second(timestamp);
        let (years, months, days) = calculate_year_month_day(days);

        // Console out: 2021-08-01 16:00:00
        format!("{:4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", years, months, days, hours, minutes, seconds)

        // todo: Change the console out to Mon, 27 Jul 2009 12:28:53 GMT (RFC 1123)
        // todo: Check https://learn.microsoft.com/en-us/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern?view=net-7.0
        /*
        format!(
            "{:0>2} {:0>2} {:0>2} {:0>2} {:0>2} {:0>2}",
             years, months, days, hours, minutes, seconds
        )
        */
    }


    /// Returns the HTTP response as a string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use http_response::HttpResponse;
    /// 
    /// let response = HttpResponse::new_1_1(HttpStatus::Ok, "Hello World!".to_string());
    /// println!("{}", response.to_string());
    /// 
    /// // HTTP/1.1 200 OK
    /// // Date: Mon, 27 Jul 2009 12:28:53 GMT
    /// // Server: Apache/2.2.14 (Win32)
    /// // Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
    /// // Content-Length: 88
    /// // Content-Type: text/html
    /// // Connection: Closed
    /// ```
    pub fn to_string(&self) -> String {
        format!("{} {} {}\r\nDate: {}\r\nServer: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: {}\r\n\r\n{}", 
            self.http_version, self.status.code(), self.status.message(), // HTTP/1.1 200 OK
            Self::now_hour_minute_second(),  // Date: Mon, 27 Jul 2009 12:28:53 GMT
            "Rust Server",  // Server: Apache/2.2.14 (Win32)
            self.body.len(),  // Content-Length: 88
            "text/html",  // Content-Type: text/html
            "Closed",  // Represents the connection type
            self.body
        )
    }

}
