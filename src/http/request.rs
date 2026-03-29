use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

use super::query_string::QueryString;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // * EXAMPLE: GET /search?name=abc&sort=1 HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // * NOTE: 아래와 동일하게 작동
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // * NOTE: 아래와 동일하게 작동
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // * NOTE: 위와 동일하게 작동하지만 자주 사용되는 패턴
        let request = str::from_utf8(buf)?;

        // 아래 request 변수 재선언을 통해 위에서 선언한 request 변수를 덮어씌움 이를 변수 섀도잉이라고 함.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // * NOTE: 위와 동일하게 작동
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // * NOTE: 아래와 동일하게 작동
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i].to_string();
        //     }
        //     None => {}
        // }

        // * NOTE: 아래와 동일하게 작동
        // let q = path.find("?");
        // if (q.is_some()) {
        //     query_string = Some(&path[q.unwrap() + 1..]);
        //     path = &path[..q.unwrap()];
        // }

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
    // * NOTE:위와 동일하게 작동
    // let mut iter =  request.chars();
    // loop {
    //     let item = iter.next()?;
    //     match item {
    //         Some(c) => if c == ' ' || c == '\r' {
    //             return Some((&request[..i], &request[i + 1..]));
    //         }
    //         None => break,
    //     }
    // }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
