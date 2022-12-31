use crate::http::response::Response;
use crate::http::{response, ParseError, Request, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Faild to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

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

    pub fn run(self, mut handler: impl Handler) -> ! {
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0u8; 1024];

                    match stream.read(&mut buffer) {
                        Ok(s) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
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
