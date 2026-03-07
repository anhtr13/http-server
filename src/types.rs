use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("not a http method")]
    UnknowMethod,
    #[error("cannot parse input")]
    InvalidInput,
}

#[derive(Debug)]
pub enum HttpMethod {
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
            _ => Err(ParseError::UnknowMethod),
        }
    }
}
