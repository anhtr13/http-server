#[allow(unused_imports)]
use std::fmt::Display;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
    net::TcpListener,
    str::FromStr,
};

#[derive(Debug)]
enum ParseError {
    InvalidInput,
}

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl FromStr for HttpMethod {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            _ => Err(ParseError::InvalidInput),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn parse(data: Vec<String>) -> Result<Self, ParseError> {
        if data.is_empty() {
            return Err(ParseError::InvalidInput);
        }
        let info: Vec<&str> = data[0].split_whitespace().collect();
        if info.len() < 3 {
            return Err(ParseError::InvalidInput);
        }
        let method = HttpMethod::from_str(&info[0])?;
        Ok(Self {
            method,
            path: info[1].to_string(),
            headers: HashMap::new(),
        })
    }
}

#[derive(Debug)]
struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
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
    fn default() -> Self {
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

fn main() -> io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&stream);
                let data: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                println!("{:?}", data);

                if let Ok(http_request) = HttpRequest::parse(data) {
                    println!("Accepted new connection: {:?}", http_request);
                    let mut response = HttpResponse::default();
                    match http_request.path.as_str() {
                        "/" => stream.write_all(response.to_string().as_bytes())?,
                        p => {
                            if p.starts_with("/echo/")
                                && let sub_path = p.trim_start_matches("/echo/")
                                && !sub_path.is_empty()
                            {
                                response.body = sub_path.to_string();
                                response.headers = HashMap::from([
                                    ("Content-Type".to_string(), "text/plain".to_string()),
                                    ("Content-Length".to_string(), sub_path.len().to_string()),
                                ]);
                                stream.write_all(response.to_string().as_bytes())?
                            } else {
                                response.status = 404;
                                stream.write_all(response.to_string().as_bytes())?
                            }
                        }
                    }
                    stream.flush()?;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
