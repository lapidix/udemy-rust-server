#![allow(dead_code)]

mod http;
mod server;

use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:3000".to_string());
    server.run();
}
