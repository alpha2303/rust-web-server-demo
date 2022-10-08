use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use crate::rusty_server::http::{HttpMethod, HttpRequest, HttpResponse};
use crate::rusty_server::router::Router;
use crate::rusty_server::thread_pool::ThreadPool;

pub struct Server {
    listener: TcpListener,
    router: Router,
    host: String,
    port: u16,
    thread_pool: ThreadPool,
    no_of_threads: usize,
}

impl Server {
    pub fn create_server(host: &str, port: u16, thread_count: usize) -> Self {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
        let mut no_of_threads = thread_count;
        if no_of_threads > 10 {
            no_of_threads = 10;
        }
        if no_of_threads == 0 {
            no_of_threads = 1;
        }

        Server {
            listener: listener,
            host: host.to_string(),
            port: port,
            router: Router::new(),
            thread_pool: ThreadPool::new(no_of_threads),
            no_of_threads: no_of_threads,
        }
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream: TcpStream = stream.unwrap();
            let router = self.router.clone();
            self.thread_pool.execute(|| {
                Server::handle_connection(stream, router);
            });
        }
    }

    pub fn add_route(
        &mut self,
        uri: &str,
        method: HttpMethod,
        callback: fn(&mut HttpRequest) -> HttpResponse,
    ) {
        self.router
            .add_route(uri, method, callback);
    }

    fn handle_connection(mut stream: TcpStream, router: Router) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let raw_request: &str = from_utf8(&buffer).unwrap();
        let mut request: HttpRequest = HttpRequest::from_request(raw_request);
        println!("{:?}", request);
        let response: HttpResponse = router.execute(&mut request);

        stream.write(response.response_body().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
