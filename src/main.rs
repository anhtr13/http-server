mod handlers;
mod request;
mod response;
mod thread_pool;
mod types;

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use request::HttpRequest;

use crate::thread_pool::ThreadPool;

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let data: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = HttpRequest::parse(&data)?;
    println!("Accepted new connection: {:?}", request);
    match request.path.as_str() {
        "/" => handlers::hander_default(&mut stream, &request)?,
        "/user-agent" => handlers::hander_user_agent(&mut stream, &request)?,
        p if p.starts_with("/echo") => handlers::hander_echo(&mut stream, &request)?,
        p if p.starts_with("/files") => handlers::hander_return_file(&mut stream, &request)?,
        _ => handlers::hander_not_found(&mut stream, &request)?,
    }
    stream.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.excute(|| {
                    if let Err(e) = handle_connection(stream) {
                        println!("Error when handling connection: {e}");
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
