use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self { Self { addr } }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>IT WORKS!!!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Error:\n {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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