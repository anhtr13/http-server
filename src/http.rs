pub mod request;
pub mod response;

use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("not a http method")]
    UnknowMethod,
    #[error("not a http header")]
    UnknowHeader,
    #[error("cannot parse input")]
    InvalidInput,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl FromStr for Method {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            _ => Err(ParseError::UnknowMethod),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Header {
    Accept,
    Authorization,
    CacheControl,
    ContentType,
    ContentLength,
    Cookie,
    Host,
    UserAgent,
}

impl FromStr for Header {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Accept" => Ok(Self::Accept),
            "Authorization" => Ok(Self::Authorization),
            "Cache-Control" => Ok(Self::CacheControl),
            "Content-Type" => Ok(Self::ContentType),
            "Content-Length" => Ok(Self::ContentLength),
            "Set-Cookie" => Ok(Self::Cookie),
            "Host" => Ok(Self::Host),
            "User-Agent" => Ok(Self::UserAgent),
            _ => Err(ParseError::UnknowHeader),
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Accept => write!(f, "Accept"),
            Self::Authorization => write!(f, "Authorization"),
            Self::CacheControl => write!(f, "Cache-Control"),
            Self::ContentType => write!(f, "Content-Type"),
            Self::ContentLength => write!(f, "Content-Length"),
            Self::Cookie => write!(f, "Set-Cookie"),
            Self::Host => write!(f, "Host"),
            Self::UserAgent => write!(f, "User-Agent"),
        }
    }
}
