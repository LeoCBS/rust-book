use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection established");
        handler_connection(stream);
    }
}

fn handler_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let file_name = "hello.html";
        //String implements Deref trait and can passing as parameter &str (smart pointer)
        write_response(file_name, status_line, stream);
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let file_name = "404.html";
        write_response(file_name, status_line, stream);
    }
}

fn write_response(file_name: &str, status_line: &str, mut stream: TcpStream) {
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
