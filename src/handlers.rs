use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use flate2::{Compression, write::GzEncoder};

use crate::http::{COMPRESSION_SCHEMES, Header, request::Request, response::Response};

pub fn hander_default(_req: Request) -> anyhow::Result<Response> {
    let response = Response::default();
    Ok(response)
}

pub fn hander_echo(req: Request) -> anyhow::Result<Response> {
    let echo_str = req.path.trim_start_matches("/echo/");
    let mut response = Response {
        status: 200,
        body: echo_str.as_bytes().to_vec(),
        headers: HashMap::from([
            (Header::ContentType, "text/plain".to_string()),
            (Header::ContentLength, echo_str.len().to_string()),
        ]),
    };
    if let Some(schemes) = req.headers.get(&Header::AcceptEncoding) {
        let schemes: Vec<&str> = schemes.split(", ").collect();
        for scheme in schemes {
            if COMPRESSION_SCHEMES.contains(&scheme) {
                response.headers.insert(Header::ContentEncoding, scheme.to_string());
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(echo_str.as_bytes())?;
                let compressed_data = encoder.finish()?;
                response
                    .headers
                    .insert(Header::ContentLength, compressed_data.len().to_string());
                response.body = compressed_data;
                break;
            }
        }
    }
    Ok(response)
}

pub fn hander_user_agent(mut req: Request) -> anyhow::Result<Response> {
    let res_body = req
        .headers
        .remove(&Header::UserAgent)
        .unwrap_or("".to_string())
        .into_bytes();
    Ok(Response {
        status: 200,
        headers: HashMap::from([
            (Header::ContentType, "text/plain".to_string()),
            (Header::ContentLength, res_body.len().to_string()),
        ]),
        body: res_body,
    })
}

pub fn hander_not_found(_req: Request) -> anyhow::Result<Response> {
    Ok(Response {
        status: 404,
        body: Vec::new(),
        headers: HashMap::new(),
    })
}

pub fn hander_read_file(req: Request) -> anyhow::Result<Response> {
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

    match fs::read(path) {
        Ok(content) => Ok(Response {
            status: 200,
            headers: HashMap::from([
                (Header::ContentType, "application/octet-stream".to_string()),
                (Header::ContentLength, content.len().to_string()),
            ]),
            body: content,
        }),
        Err(_) => Ok(Response {
            status: 404,
            body: Vec::new(),
            headers: HashMap::new(),
        }),
    }
}
pub fn hander_write_file(req: Request) -> anyhow::Result<Response> {
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
    write!(&mut file, "{}", str::from_utf8(&req.body)?)?;
    Ok(Response {
        status: 201,
        headers: HashMap::new(),
        body: Vec::new(),
    })
}
