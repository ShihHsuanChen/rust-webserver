use std::io::Write;
use std::net::TcpStream;

use crate::request::Request;
use crate::http::PROTOCOL;
use crate::response::{make_response, Response};
use crate::router::Router;


pub struct App<'a> {
    router: Router<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self { router: Router::new() }
    }

    pub fn include_router(&mut self, prefix: &str, router: Box<Router<'a>>) {
        self.router.include_router(prefix, router);
    }

    pub fn route(&self, request: &Request) -> Option<Response> {
        self.router.route(&request.path.to_string(), request)
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        println!("Connection established");
        let request = match Request::from_stream(&stream) {
            Ok(v) => v,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };
        println!("{}", request);

        // let args = HashMap::<String, String>::new();

        let default_response = make_response(404, String::from("Not found"));
        let response = {
            if request.protocol != PROTOCOL::HTTP_1_1 {
                default_response
            } else if let Some(resp) = self.route(&request) {
                resp
            } else {
                default_response
            }
        };
        if let Err(e) = stream.write_all(response.as_string().as_bytes()) {
            println!("Fail to response: {e:?}");
        }
    }
}
