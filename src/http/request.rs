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
use super::HttpMethod;


#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod, 
        url: impl Into<String>, 
        headers: Vec<String>, 
        body: String
    ) -> HttpRequest {
        HttpRequest {
            method, 
            url: url.into(), 
            headers, 
            body
        }
    }

    pub fn new_1_1(
        method: HttpMethod, 
        url: impl Into<String>, 
        body: String
    ) -> HttpRequest {
        HttpRequest {
            method, 
            url: url.into(),
            headers: Vec::new(), 
            body
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
    /// let request = HttpRequest::new_1_1(HttpMethod::Get, "/index.html", "Hello World!".to_string());
    /// println!("{}", request.to_string());
    /// ```
    fn to_string(&self) -> String {
        let mut request = format!("{:?} {} HTTP/1.1\r\n", self.method, self.url);
        for header in &self.headers {
            request.push_str(&format!("{}\r\n", header));
        }
        request.push_str(&format!("\r\n{}", self.body));
        request
    }
}