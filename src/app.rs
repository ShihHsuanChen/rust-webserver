use std::net::TcpStream;

use crate::request::Request;
use crate::http::PROTOCOL;
use crate::response::{make_text_response, MakeResponse};
use crate::router::Router;
use crate::router::ResponseResult;


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

    pub fn route(&self, request: &Request) -> Option<ResponseResult> {
        self.router.route(&request.path.to_string(), request)
    }

    pub fn handle_connection(&self, stream: TcpStream) -> Result<(),String> {
        println!("Connection established");
        let request = Request::from_stream(&stream)?;
        println!("{}", request);
        // TODO: validate from schema
        // TODO: handle validation error

        // let args = HashMap::<String, String>::new();

        let default_response = make_text_response(404, String::from("Not found"))?;
        if request.protocol != PROTOCOL::HTTP_1_1 {
            match default_response.write(stream) {
                _ => (),
            };
        } else if let Some(resp) = self.route(&request) {
            let resp = resp?;
            // TODO: error to error response
            match resp.write(stream) {
                _ => (),
            };
        } else {
            match default_response.write(stream) {
                _ => (),
            };
        }
        Ok(())
    }
}
