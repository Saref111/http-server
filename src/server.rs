use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
    host: String,
    port: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        let url: Vec<&str> = address.split(':').collect();

        let mut host: String = String::from("localhost");
        let mut port: String = String::from("8888");

        for (i, &a) in url.iter().enumerate() {
            match i {
                0 => host = a.to_owned(),
                1 => port = a.to_owned(),
                _ => break,
            }
        }

        Self {
            address,
            host,
            port,
        }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0u8; 1024];

                    match stream.read(&mut buffer) {
                        Ok(s) => {
                            println!("{s}");
                            println!("request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(err) => {
                                    println!("Failed to parse the request: {err}")
                                }
                            }
                        }
                        Err(e) => println!("Failed to read {}", e),
                    }
                }
                Err(e) => println!("Failed to connect {}", e),
            }
        }
    }
}
