const HTTP_INDEX: &str = "hello.html";
const NOT_FOUND_PAGE: &str = "404.html";

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use log::{info};
use simple_logger::SimpleLogger;
use std::{thread, time::Duration};
use rust_web_server::ThreadPool;

fn main() {
    SimpleLogger::new().init().unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    info!("{request_line}");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", HTTP_INDEX),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", HTTP_INDEX)
        },
        _ => ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_PAGE),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

}
