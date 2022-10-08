mod rusty_server;

use crate::rusty_server::helpers::methods::get_file_contents;
use crate::rusty_server::http::{HttpMethod, HttpResponse};
use crate::rusty_server::server::Server;

fn main() {
    let mut web_server: Server = Server::create_server("127.0.0.1", 7878, 4);

    web_server.add_route("/", HttpMethod::Get, |request| {
        let headers = request.get_headers().clone();
        let contents = get_file_contents("src/templates/hello.html");
        HttpResponse::new(200, Some(headers), contents.as_str())
    });
    web_server.start();
}
