mod handlers;
mod http;
mod thread_pool;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http::request::Request;

use crate::{
    http::{Header, Method},
    thread_pool::ThreadPool,
};

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    loop {
        let request = Request::from_tcpstream(&mut stream)?;
        println!("New request: {:?}", request);

        let should_close = if let Some(closure) = request.headers.get(&http::Header::Connection)
            && closure == "close"
        {
            true
        } else {
            false
        };

        let mut response = match (&request.method, request.path.as_str()) {
            (_, "/") => handlers::hander_default(request)?,
            (_, p) if p.starts_with("/echo") => handlers::hander_echo(request)?,
            (Method::Get, "/user-agent") => handlers::hander_user_agent(request)?,
            (Method::Get, p) if p.starts_with("/files") => handlers::hander_read_file(request)?,
            (Method::Post, p) if p.starts_with("/files") => handlers::hander_write_file(request)?,
            _ => handlers::hander_not_found(request)?,
        };

        if should_close {
            response.headers.insert(Header::Connection, "close".to_string());
        }

        stream.write_all(&response.into_bytes())?;
        stream.flush()?;

        if should_close {
            return Ok(());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(8);

    println!("Logs from program:");

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
