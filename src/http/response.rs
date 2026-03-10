use std::collections::HashMap;

use crate::http::Header;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<Header, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn default() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
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

    pub fn into_bytes(mut self) -> Vec<u8> {
        let mut bytes = format!("HTTP/1.1 {} {}\r\n", self.status, self.status_reason()).into_bytes();
        for (key, val) in self.headers {
            bytes.append(&mut format!("{}: {}\r\n", key, val).into_bytes());
        }
        bytes.append(&mut vec![b'\r', b'\n']);
        bytes.append(&mut self.body);
        bytes
    }
}
