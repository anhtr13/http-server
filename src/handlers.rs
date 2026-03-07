use std::{collections::HashMap, env, fs, io::Write, net::TcpStream, path::Path};

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

pub fn hander_return_file(stream: &mut TcpStream, req: &HttpRequest) -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut dir_name = String::new();
    let mut dir_flag = false;
    for arg in args {
        if arg == "--directory" {
            dir_flag = true;
            continue;
        }
        if dir_flag {
            dir_name = arg;
            break;
        }
    }

    let file_name = req.path.trim_start_matches("/files/");
    let path_str = format!("{dir_name}/{file_name}");
    let path = Path::new(&path_str);

    match fs::read_to_string(path) {
        Ok(content) => {
            let response = HttpResponse {
                status: 200,
                headers: HashMap::from([
                    (
                        "Content-Type".to_string(),
                        "application/octet-stream".to_string(),
                    ),
                    ("Content-Length".to_string(), content.len().to_string()),
                ]),
                body: content,
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
        Err(_) => {
            let response = HttpResponse {
                status: 404,
                body: String::new(),
                headers: HashMap::new(),
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
    }

    Ok(())
}
