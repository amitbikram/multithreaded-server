use std::io::prelude::*;
use std::fs;
use std::{thread, time};
use hello::ThreadPool;

use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pools = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pools.execute(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    let (html, contentTmpl) = if buffer.starts_with(get) {
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    let contents = fs::read_to_string(html).unwrap();
    let response = format!(
        "{}{}",
        contentTmpl,
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
