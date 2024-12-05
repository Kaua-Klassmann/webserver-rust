use std::fs;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

fn main() {
    let listener: TcpListener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(tcp) => tcp,
        Err(_) => panic!("Port 3000 is blocked!")
    };

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handler_connection(stream);
    }
}

fn handler_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = 
        if buffer.starts_with(b"GET / ") {
            ("200 OK", "index.html")
        } else {
            ("404 NOT FOUND", "404.html")
        };

    let contents: String = fs::read_to_string(format!("src/{}", filename)).unwrap();
    let response: String = format!(
        "HTTP/3 {}\nContent-Length: {}\n\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}