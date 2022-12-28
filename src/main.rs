fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn run(&self) {
        todo!()
    }
}
