use std::{collections::HashMap, str::FromStr};

use crate::types::{HttpMethod, ParseError};

#[allow(unused)]
#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(data: &[String]) -> anyhow::Result<Self> {
        if data.is_empty() {
            return Err(anyhow::Error::new(ParseError::InvalidInput));
        }

        let req_status: Vec<&str> = data[0].split_whitespace().collect();
        if req_status.len() < 3 {
            return Err(ParseError::InvalidInput.into());
        }

        let method = HttpMethod::from_str(req_status[0])?;
        let path = req_status[1].to_string();
        let mut headers = HashMap::new();
        let mut body = String::new();

        for line in data.iter().skip(1) {
            if let Some((key, val)) = line.split_once(": ") {
                headers.insert(key.to_string(), val.to_string());
            } else {
                body.push_str(line);
                body.push_str("\r\n");
            }
        }

        Ok(Self {
            method,
            path,
            headers,
            body,
        })
    }
}
