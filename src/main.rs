#[allow(unused_imports)]
use std::fmt::Display;
use std::{
    io::{self, Write},
    net::TcpListener,
};

#[derive(Debug)]
struct HttpResponse {
    status: u16,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n",
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
                println!("accepted new connection");
                let response = HttpResponse::new(200);
                stream.write_all(response.to_string().as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
