use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_line = format!("HTTP/1.1 {} {}", self.status, self.status_reason());
        let mut headers = String::new();
        for (key, val) in self.headers.iter() {
            headers.push_str(&format!("{}: {}\r\n", key, val));
        }
        write!(f, "{}\r\n{}\r\n{}\r\n", status_line, headers, self.body)
    }
}

impl HttpResponse {
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
            404 => "Not Found",
            _ => "",
        }
    }
}
