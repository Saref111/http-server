fn main() {
    let address = String::from("127.0.0.1:8080");

    let server = Server::new(address);
    server.run();
}

struct Server {
    address: String,
    path: String,
    port: String,
}

impl Server {
    fn new(address: String) -> Self {
        let url: Vec<&str> = address.split(':').collect();
        let mut path: String = String::from("localhost");
        let mut port: String = String::from("8888");

        for (i, &a) in url.iter().enumerate() {
            match i {
                0 => path = a.to_owned(),
                1 => port = a.to_owned(),
                _ => break,
            }
        }
        Self {
            address,
            path,
            port,
        }
    }

    fn run(&self) {
        todo!()
    }
}
