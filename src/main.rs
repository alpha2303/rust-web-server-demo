mod components;

use crate::components::server::Server;

fn main() {
    let web_server: Server = Server::create_server("127.0.0.1", 7878, 4);
    web_server.start();
}
