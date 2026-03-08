use std::{collections::HashMap, fmt::Display};

use crate::http::Header;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<Header, String>,
    pub body: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_line = format!("HTTP/1.1 {} {}", self.status, self.status_reason());
        let mut headers = String::new();
        for (key, val) in self.headers.iter() {
            headers.push_str(&format!("{}: {}\r\n", key, val));
        }
        write!(f, "{}\r\n{}\r\n{}\r\n", status_line, headers, self.body)
    }
}

impl Response {
    pub fn default() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    fn status_reason(&self) -> &str {
        match self.status {
            200 => "OK",
            201 => "Created",
            404 => "Not Found",
            _ => "",
        }
    }
}
