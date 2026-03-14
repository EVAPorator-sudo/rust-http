use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("172.0.0.1:8080").unwrap();
    println!("listening at 172.0.0.1:8080");
}
