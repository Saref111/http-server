use super::method::Method;
use ::std::error::Error;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::{from_utf8, Utf8Error},
};

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    /*
    GET /users?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY
    */
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        let request = from_utf8(buf)?;
        unimplemented!()
    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' {
            return Some((&req[..i], &req[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidRequest => "Invalid request",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}
