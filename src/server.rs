use std::alloc::handle_alloc_error;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{Request, Response, StatusCode};
use crate::http::request::ParseError;

pub struct Server {
    addr: String
}

pub trait Handler {

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Error occurred: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }

    fn handle_request(&mut self, request: &Request) -> Response;
}

impl Server {
    pub fn new(addr: String) -> Self { Self { addr } }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }

                        Err(e) => println!("Failed to read a connection: {:#?}", e)
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}