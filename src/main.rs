mod handlers;
mod request;
mod response;
mod types;

use request::HttpRequest;
#[allow(unused_imports)]
use std::fmt::Display;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let data: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Ok(request) = HttpRequest::parse(&data) {
        println!("Accepted new connection: {:?}", request);
        match request.path.as_str() {
            "/" => handlers::hander_default(&mut stream, &request)?,
            "/user-agent" => handlers::hander_user_agent(&mut stream, &request)?,
            p if p.starts_with("/echo") => handlers::hander_echo(&mut stream, &request)?,
            _ => handlers::hander_not_found(&mut stream, &request)?,
        }
        stream.flush()?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
