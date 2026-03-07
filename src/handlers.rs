use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::{request::HttpRequest, response::HttpResponse};

pub fn hander_default(stream: &mut TcpStream, _req: &HttpRequest) -> anyhow::Result<()> {
    let response = HttpResponse::default();
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_echo(stream: &mut TcpStream, req: &HttpRequest) -> anyhow::Result<()> {
    let sub_path = req.path.trim_start_matches("/echo/");
    let response = HttpResponse {
        status: 200,
        body: sub_path.to_string(),
        headers: HashMap::from([
            ("Content-Type".to_string(), "text/plain".to_string()),
            ("Content-Length".to_string(), sub_path.len().to_string()),
        ]),
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_user_agent(stream: &mut TcpStream, req: &HttpRequest) -> anyhow::Result<()> {
    let res_body = req
        .headers
        .get("User-Agent")
        .unwrap_or(&"".to_string())
        .to_string();
    let res_headers = HashMap::from([
        ("Content-Type".to_string(), "text/plain".to_string()),
        ("Content-Length".to_string(), res_body.len().to_string()),
    ]);
    let response = HttpResponse {
        status: 200,
        body: res_body,
        headers: res_headers,
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_not_found(stream: &mut TcpStream, _req: &HttpRequest) -> anyhow::Result<()> {
    let response = HttpResponse {
        status: 404,
        body: String::new(),
        headers: HashMap::new(),
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}
