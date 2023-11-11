//! This module defines types and methods for working with HTTP requests and responses.
//! 
//! 
// #![cfg(feature = "http")]  // Only compile this module if the "codex" feature is enabled
// to add this on the Cargo.toml file, add the following line:
// [features]
// http = ["http"]


pub mod request;
pub mod response;

// * HTTP Status Codes ---------------------------------------------------------------------------------------------

/// Define a macro called 'impl_http_status_enum' that takes two arguments:
/// 1. $enum_name: the name of an enum type, and
/// 2. a sequence of 'variant' definitions, each containing:
///    - $variant: the variant name,
///    - $value: the numerical value associated with the variant,
///    - $message: a string message associated with the variant.
macro_rules! impl_http_status_enum {
    ($enum_name:ident; $($variant:ident => ($value:expr, $message:expr)),* $(,)?) => {
        impl $enum_name {  // Create a set of methods for the specified enum type ($enum_name).

            /// Returns the numerical value associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - [u16] - The numerical value associated with the enum variant.
            pub fn code(&self) -> u16 {  // Define a 'code' method that returns a u16.
                // Match the value of 'self' (an instance of the enum) to its variants.
                match self {$($enum_name::$variant => $value,)*}
            }


            /// Returns the message associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - `&str` - The message associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - `&str` - The message associated with the enum variant.
            pub fn message(&self) -> &str {  // Define a 'message' method that returns a string message.
                // Match the value of 'self' to its variants and return the associated $message.
                match self {$($enum_name::$variant => $message,)*}
            }

            /// Returns the enum variant associated with the given [u16] value.
            /// 
            /// # Arguments
            /// 
            /// - `value` - The [u16] value to match against the enum variants.
            /// 
            /// # Returns
            /// 
            /// - `Some(enum_variant)` - If the value matches one of the enum variants.
            pub fn from_u16(value: u16) -> Option<Self> {  // Define a 'from_u16' method that takes a u16 and returns an Option of the enum.
                match value {  // Match the provided 'value' to the $value associated with each variant.
                    $($value => Some($enum_name::$variant),)*  // If there's a match, return Some(enum_variant).
                    _ => None,  // If there's no match, return None.
                }
            }

        }
    };
}


// Define a macro called 'http_status_enum' that takes a sequence of 'variant' definitions,
// each containing the variant name ($variant), the numerical value ($value), and a message ($message).
macro_rules! http_status_enum {
    ($($variant:ident => ($value:expr, $message:expr)),* $(,)?) => {
        /// Represents a set of HTTP status codes and their associated messages.
        ///
        /// This enum allows you to work with standard HTTP status codes and their corresponding
        /// messages. It provides methods to retrieve the numerical code, the associated message,
        /// and convert a numerical value into the appropriate status code variant.
        ///
        /// # Example
        ///
        /// ```
        /// pub mod http;
        /// use http::HttpStatus;
        ///
        /// let status = HttpStatus::_200;
        /// assert_eq!(status.code(), 200);
        /// assert_eq!(status.message(), "OK");
        ///
        /// let value = 404;
        /// match HttpStatus::from_u16(value) {
        ///     Some(status) => println!("HTTP Status from u16: {:?}", status),
        ///     None => println!("Invalid HTTP Status Code"),
        /// }
        /// ```
        #[derive(
            Debug,  // Allow writting the enum to the console. (e.g. println!("{:?}", HttpStatus::_200))
            PartialEq,  // Allow the comparison of enum variants using the '==' operator.
            Clone  // Allow the cloning of enum variants.
        )]
        pub enum HttpStatus {
            $($variant,)*  // List the enum variants ($variant).
        }

        impl_http_status_enum!(HttpStatus; $($variant => ($value, $message)),*);
    };
}


// Define the HTTP status enum using the 'http_status_enum' macro.
http_status_enum!(
    // Define variants with numerical values and messages.
    // * 1XX: Informational - Request received, continuing process
    _100 => (100, "Continue"),
    _101 => (101, "Switching Protocols"),
    _102 => (102, "Processing"),

    // * 2XX: Success - The action was successfully received, understood, and accepted
    _200 => (200, "OK"),
    _201 => (201, "Created"),
    _202 => (202, "Accepted"),
    _203 => (203, "Non-Authoritative Information"),
    _204 => (204, "No Content"),
    _205 => (205, "Reset Content"),

    // * 3XX: Redirection - Further action must be taken in order to complete the request
    _300 => (300, "Multiple Choices"),
    _301 => (301, "Moved Permanently"),
    _302 => (302, "Found"),

    // * 4XX: Client Error - The request contains bad syntax or cannot be fulfilled
    _400 => (400, "Bad Request"),
    _401 => (401, "Unauthorized"),
    // _402 => (402, "Payment Required"),
    _403 => (403, "Forbidden"),
    _404 => (404, "Not Found"),
    _405 => (405, "Method Not Allowed"),
    _406 => (406, "Not Acceptable"),
    _408 => (408, "Request Timeout"),
    _409 => (409, "Conflict"),
    _410 => (410, "Gone"),

    // * 5XX: Server Error - The server failed to fulfill an apparently valid request
    _500 => (500, "Internal Server Error"),
    _501 => (501, "Not Implemented"),
    _502 => (502, "Bad Gateway"),
    _503 => (503, "Service Unavailable"),
    _504 => (504, "Gateway Timeout"),
    // _505 => (505, "HTTP Version Not Supported"),
    // _506 => (506, "Variant Also Negotiates"),
    _507 => (507, "Insufficient Storage"),
    // _508 => (508, "Loop Detected"),
    // _510 => (510, "Not Extended"),
    _511 => (511, "Network Authentication Required"),
    _599 => (599, "Network Connect Timeout Error"),
);

impl HttpStatus {
    /// Returns the enum variant associated with the given [u16] value.
    /// 
    /// # Arguments
    /// 
    /// - `value` - The [u16] value to match against the enum variants.
    /// 
    /// # Returns
    /// 
    /// - `Some(enum_variant)` - If the value matches one of the enum variants.
    pub fn new(value: u16) -> Option<Self> {
        Self::from_u16(value)
    }

}

/// Returns the default HTTP status code.
/// 
/// The default HTTP status code is [`HttpStatus::_501`] (Not Implemented).
/// This because could be used as a default response if the server is unable to handle the request.
/// 
/// # Returns
/// 
/// - [`HttpStatus`] - The default HTTP status code.
impl Default for HttpStatus {
    fn default() -> Self {
        HttpStatus::_501  // Not Implemented
    }
}

/// Impl of the [`std::fmt::Display`] trait for the [`HttpStatus`] enum.
/// 
/// This allows the enum to be printed to the console using the [`println!`] macro.
/// 
/// Returns the HTTP status code as a [`String`] in the format: `<code> <message>`.
/// 
/// The string use the ANSI escape codes to color the code based on the status code type.
/// # Colors:
/// - 1XX: Blue
/// - 2XX: Green
/// - 3XX: Magenta
/// - 4XX: Yellow
/// - 5XX: Red
impl std::fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "{} {}", self.code(), self.message())
        write!(f, "{} {}", 
            format!("\x1b[{}m{}\x1b[0m", 
                match self.code() {
                    100..=199 => "34",  // Blue
                    200..=299 => "32",  // Green
                    300..=399 => "35",  // Magenta
                    400..=499 => "33",  // Yellow
                    500..=599 => "31",  // Red
                    _ => "0",  // Default (Theoricaly, this should never happen)
                }, &self.code()), 
            self.message())
    }
}


// * HTTP Methods -------------------------------------------------------------------------------------------------

/// Define a macro called 'http_methods_enum' that generates an enum for HTTP methods.
macro_rules! http_methods_enum {
    ($($variant:ident),*) => {
        /// Represents HTTP methods (GET, POST, PUT, DELETE, etc.).
        ///
        /// This enum provides a set of HTTP methods that can be used to specify the desired action
        /// when making HTTP requests. It allows you to work with standard HTTP methods and convert
        /// between their enum representation and string representation.
        #[derive(Debug, PartialEq, Clone)]
        pub enum HttpMethod {
            $($variant,)*
        }

        impl HttpMethod {
            /// Returns a string representation of the HTTP method.
            ///
            /// # Returns
            ///
            /// - `&str` - A string representation of the HTTP method.
            pub fn as_str(&self) -> &str {
                match self {
                    $(HttpMethod::$variant => stringify!($variant),)*
                }
            }

            /// Returns the HTTP method from a string representation.
            /// Returns None for unsupported methods.
            ///
            /// # Arguments
            ///
            /// - `method` - A string representation of the HTTP method.
            ///
            /// # Returns
            ///
            /// - `Some(HttpMethod)` - If the string represents a valid HTTP method.
            /// - `None` - If the string does not match any supported HTTP method.
            pub fn from_str(method: &str) -> Option<Self> {
                match method {
                    $(stringify!($variant) => Some(HttpMethod::$variant),)*
                    _ => None,
                }
            }
        }
    };
}


// Generate the HTTP methods enum and methods using the macro.
http_methods_enum!(GET, POST, PUT, DELETE);

impl Default for HttpMethod {
    /// Returns the default HTTP method, which is [`HttpMethod::GET`].
    ///
    /// The default method is typically used when an HTTP request is made without explicitly
    /// specifying a method.
    ///
    /// # Returns
    ///
    /// - [`HttpMethod`] - The default HTTP method.
    fn default() -> Self {
        HttpMethod::GET
    }
}


// * HTTP Versions ------------------------------------------------------------------------------------------------

// Define the HTTP versions using a macro
macro_rules! http_versions_enum {
    ($($variant:ident ($str:expr)),*) => {
        /// Represents HTTP versions (HTTP/1.0, HTTP/1.1, HTTP/2.0).
        ///
        /// This enum provides a set of HTTP versions that can be used to specify the desired version
        /// when making HTTP requests. It allows you to work with standard HTTP versions and convert
        /// between their enum representation and string representation.
        #[derive(Debug, PartialEq, Clone)]
        pub enum HttpVersion {
            $($variant,)*
        }

        impl HttpVersion {
            /// Returns a string representation of the HTTP version.
            ///
            /// # Returns
            ///
            /// - `&str` - A string representation of the HTTP version.
            pub fn as_str(&self) -> &str {
                match self {
                    $(HttpVersion::$variant => $str,)*
                }
            }

            /// Returns the HTTP version from a string representation.
            /// Returns None for unsupported versions.
            ///
            /// # Arguments
            ///
            /// - `version` - A string representation of the HTTP version.
            ///
            /// # Returns
            ///
            /// - `Some(HttpVersion)` - If the string represents a valid HTTP version.
            /// - `None` - If the string does not match any supported HTTP version.
            pub fn from_str(version: &str) -> Option<Self> {
                match version {
                    $($str => Some(HttpVersion::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

// Example usage:
http_versions_enum!(
    Http1_0("HTTP/1.0"),
    Http1_1("HTTP/1.1"),
    Http2_0("HTTP/2.0")
);

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for HttpVersion {
    /// Returns the default HTTP version, which is [`HttpVersion::Http1_1`].
    ///
    /// The default version is typically used when an HTTP request is made without explicitly
    /// specifying a version.
    fn default() -> Self {
        HttpVersion::Http1_1
    }
}
