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

#[derive(Debug)]
struct HttpRequest {
    _method: HttpMethod,
    path: String,
    _headers: HashMap<String, String>,
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
            _method: method,
            path: info[1].to_string(),
            _headers: HashMap::new(),
        })
    }
}

#[derive(Debug)]
struct HttpResponse {
    status: u16,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n\r\n",
            self.status,
            self.status_reason()
        )
    }
}

impl HttpResponse {
    fn new(status: u16) -> Self {
        Self { status }
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
                    match http_request.path.as_str() {
                        "/" => stream.write_all(HttpResponse::new(200).to_string().as_bytes())?,
                        _ => stream.write_all(HttpResponse::new(404).to_string().as_bytes())?,
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
