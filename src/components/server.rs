use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::components::methods::HttpResponse;
use crate::components::thread_pool::ThreadPool;

pub struct Server {
    listener: TcpListener,
    host: String,
    port: u16,
    thread_pool: ThreadPool,
    no_of_threads: usize,
}

impl Server {
    pub fn create_server(host: &str, port: u16, thread_count: usize) -> Self {
        let listener: TcpListener =
            TcpListener::bind(format!("{}:{}", host, port.to_string().as_str())).unwrap();
        let mut no_of_threads = thread_count;
        if no_of_threads > 10 {
            no_of_threads = 10;
        }

        Server {
            listener: listener,
            host: host.to_string(),
            port: port,
            thread_pool: ThreadPool::new(no_of_threads),
            no_of_threads: no_of_threads,
        }
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream: TcpStream = stream.unwrap();
            self.thread_pool.execute(|| {
                Server::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let (status, headers, content_path) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            ("200", "headers", "src/templates/hello.html")
        } else {
            ("404", "headers", "src/templates/notfound.html")
        };

        let contents = fs::read_to_string(content_path).unwrap();
        let response_string: String = Server::create_response(status, headers, contents.as_str());

        stream.write(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn create_response(status: &str, headers: &str, contents: &str) -> String {
        let http_response: HttpResponse = HttpResponse::new(status, headers, contents);
        http_response.response_body()
    }
}
