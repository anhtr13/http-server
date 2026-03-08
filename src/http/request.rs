use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

use crate::http::{Header, Method, ParseError};

#[allow(unused)]
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<Header, String>,
    pub body: Vec<u8>,
}

impl Request {
    fn from_utf8(data: Vec<u8>) -> anyhow::Result<Self> {
        if data.is_empty() {
            return Err(anyhow::Error::new(ParseError::InvalidInput));
        }

        let mut start = 0;
        let mut lines = Vec::new();
        for (i, win) in data.windows(2).enumerate() {
            if win == [b'\r', b'\n']
                && start < i
                && let slice = &data[start..i]
                && let Ok(line) = str::from_utf8(slice)
            {
                lines.push(line);
                start = i + 2;
            }
        }

        if lines.is_empty() {
            return Err(anyhow::Error::new(ParseError::InvalidInput));
        }

        let req_status: Vec<&str> = lines[0].split_whitespace().collect();
        if req_status.len() < 3 {
            return Err(ParseError::InvalidInput.into());
        }

        let method = Method::from_str(req_status[0])?;
        let path = req_status[1].to_string();
        let mut headers = HashMap::new();

        for line in lines.iter().skip(1) {
            if let Some((key, val)) = line.split_once(": ")
                && let Ok(header) = Header::from_str(key)
            {
                headers.insert(header, val.to_string());
            }
        }

        Ok(Self {
            method,
            path,
            headers,
            body: Vec::new(),
        })
    }

    pub fn from_tcpstream(stream: &mut TcpStream) -> anyhow::Result<Self> {
        let mut reader = BufReader::new(stream);
        let mut buffer = Vec::new();

        while let Ok(n) = reader.read_until(b'\n', &mut buffer) {
            if n == 2 && buffer[buffer.len() - 2..] == [b'\r', b'\n'] {
                break;
            }
        }

        let mut request = Self::from_utf8(buffer)?;
        if let Some(content_lenth) = request.headers.get(&Header::ContentLength)
            && let Ok(n) = content_lenth.parse::<usize>()
        {
            let mut buffer = vec![0u8; n];
            reader.read_exact(&mut buffer)?;
            request.body = buffer;
        }

        Ok(request)
    }
}
