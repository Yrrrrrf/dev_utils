//! HTTP Request
//! 
//! A HTTP request is a message sent from a client to a server. It contains a request line, headers, and a body.
//! 
//! # HTTP Request
//! 
//! ![Client](https://www.tutorialspoint.com/http/http_requests.htm)
//! 
//! # Example
//! ```
//! GET /index.html HTTP/1.1
//! User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
//! Host: www.tutorialspoint.com
//! Accept-Language: en-us
//! ```
use super::{HttpMethod, HttpVersion};


#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub url: String,
    // pub headers: Vec<String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod, 
        http_version: HttpVersion, 
        url: impl Into<String>, 
        // headers: Vec<String>, 
        body: impl Into<String>
    ) -> HttpRequest {
        HttpRequest {
            method,
            http_version,
            url: url.into(),
            // headers, 
            body: body.into(),
        }
    }

}

impl ToString for HttpRequest {
    /// Returns the HTTP request as a string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use http_request::HttpRequest;
    /// use http_method::HttpMethod;
    /// 
    /// let request = HttpRequest::new(HttpMethod::GET, HttpVersion::Http1_1,"/index.html", "Hello, Rust!".to_string());
    /// println!("{}", request.to_string());
    /// ```
    fn to_string(&self) -> String {
        let mut request = format!("{:?} {} {}\r\n", self.method, self.url, self.http_version);
        // for header in &self.headers {
        //     request.push_str(&format!("{}\r\n", header));
        // }
        request.push_str(&format!("\r\n{}", self.body));
        request
    }
}
