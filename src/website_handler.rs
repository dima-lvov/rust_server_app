use std::fs;

use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }



    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}\\{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(canonized_path) => {
                if canonized_path.starts_with(format!("\\\\?\\{}", self.public_path)) {
                    fs::read_to_string(canonized_path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None)
                },
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}