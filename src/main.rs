#![allow(dead_code)]
use std::env;
use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;


fn main() {
    let public_path = env::var("PUBLIC_PATH")
        .unwrap_or_else(|_| format!("{}\\public", env!("CARGO_MANIFEST_DIR").to_string()));
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
