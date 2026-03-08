use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    net::TcpStream,
    path::Path,
};

use crate::http::{COMPRESSION_SCHEMES, Header, request::Request, response::Response};

pub fn hander_default(stream: &mut TcpStream, _req: &Request) -> anyhow::Result<()> {
    let response = Response::default();
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_echo(stream: &mut TcpStream, req: &Request) -> anyhow::Result<()> {
    let sub_path = req.path.trim_start_matches("/echo/");
    let mut response = Response {
        status: 200,
        body: sub_path.to_string(),
        headers: HashMap::from([
            (Header::ContentType, "text/plain".to_string()),
            (Header::ContentLength, sub_path.len().to_string()),
        ]),
    };
    if let Some(schemes) = req.headers.get(&Header::AcceptEncoding) {
        let schemes: Vec<&str> = schemes.split(", ").collect();
        for scheme in schemes {
            if COMPRESSION_SCHEMES.contains(&scheme) {
                response.headers.insert(Header::ContentEncoding, scheme.to_string());
                break;
            }
        }
    }
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_user_agent(stream: &mut TcpStream, req: &Request) -> anyhow::Result<()> {
    let res_body = req
        .headers
        .get(&Header::UserAgent)
        .unwrap_or(&"".to_string())
        .to_string();
    let res_headers = HashMap::from([
        (Header::ContentType, "text/plain".to_string()),
        (Header::ContentLength, res_body.len().to_string()),
    ]);
    let response = Response {
        status: 200,
        body: res_body,
        headers: res_headers,
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_not_found(stream: &mut TcpStream, _req: &Request) -> anyhow::Result<()> {
    let response = Response {
        status: 404,
        body: String::new(),
        headers: HashMap::new(),
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

pub fn hander_read_file(stream: &mut TcpStream, req: &Request) -> anyhow::Result<()> {
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
            let response = Response {
                status: 200,
                headers: HashMap::from([
                    (Header::ContentType, "application/octet-stream".to_string()),
                    (Header::ContentLength, content.len().to_string()),
                ]),
                body: content,
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
        Err(_) => {
            let response = Response {
                status: 404,
                body: String::new(),
                headers: HashMap::new(),
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
    }

    Ok(())
}
pub fn hander_write_file(stream: &mut TcpStream, req: &Request) -> anyhow::Result<()> {
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
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path_str)?;
    write!(&mut file, "{}", req.body)?;
    let response = Response {
        status: 201,
        headers: HashMap::new(),
        body: String::new(),
    };
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}
