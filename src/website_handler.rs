use std::fs;

use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(p) => {
                if p.starts_with(&self.public_path) {
                    fs::read_to_string(p).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    some => Response::new(StatusCode::Ok, some),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            Method::POST => todo!(),
            Method::DELETE => todo!(),
            Method::PUT => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
