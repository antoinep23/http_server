fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Server { addr }
    }

    fn run(&self) {
        println!("Server is running on port {}...", self.addr);
    }
}
