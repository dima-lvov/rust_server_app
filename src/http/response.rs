use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::http::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\n\r\n\r{}",
            self.status_code,
            self.status_code.reason_phrase(),
            &self.body.as_ref().unwrap_or(&String::from(""))
        )
    }
}

