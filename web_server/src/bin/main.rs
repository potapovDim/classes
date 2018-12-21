use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

extern crate web_server;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_connection(stream));
        println!("Connection established!");
    }
}
fn handle_connection(mut stream: TcpStream) {
    let index_html_path = "/Users/dpot/Documents/classes/web_server/src/index.html";
    let not_found_html_path = "/Users/dpot/Documents/classes/web_server/src/404.html";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET"; // byte literal ??
    let get_sleep = b"GET /sleep";

    println!("{}", String::from_utf8_lossy(&buffer[..])); // print request content as a string

    let (content, status) = if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            fs::read_to_string(index_html_path).unwrap(),
            "HTTP/1.1 200 OK\r\n\r\n",
        )
    } else if buffer.starts_with(get) {
        (
            fs::read_to_string(index_html_path).unwrap(),
            "HTTP/1.1 200 OK\r\n\r\n",
        )
    } else {
        (
            fs::read_to_string(not_found_html_path).unwrap(),
            "HTTP/1.1 404 Not Found\r\n\r\n",
        )
    };
    let response = format!("{}{}", status, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
