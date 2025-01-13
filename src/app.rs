use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use crate::request::Request;
use crate::http::PROTOCOL;
use crate::response::{make_response, Response};
use crate::router::Router;


pub struct App<'a> {
    router: Router<'a>,
    // routers: RoutersMap<'a>, // prefix: routers
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
}


pub fn run(app: App, host: &str, port: usize) {
    let ip_port = format!("{host}:{port}");
    let listener = match TcpListener::bind(&ip_port) {
        Ok(v) => v,
        Err(_) => {
            println!("Cannot bind {ip_port}, it is already used by other process.");
            std::process::exit(1);
        }
    };

    println!("Listening to {} ...", &ip_port);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };
        handle_connection(&app, stream);
    }
}


use std::sync::Arc;


/* TODO: implement multithreaded on App
pub fn run(app: Arc<App>, host: &str, port: usize, threads: usize) {
    let pool = ThreadPool::new(threads);
    let ip_port = format!("{host}:{port}")
    let listener = match TcpListener::bind(&ip_port) {
        Ok(v) => v,
        Err(_) => {
            println!("Cannot bind {ip_port}, it is already used by other process.");
            std::process::exit(1);
        }
    };

    println!("Listening to {} ...", &ip_port);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };
        
        pool.execute(|| {
            handle_connection(app.clone(), stream);
        });
    }
}
*/


pub fn handle_connection(app: &App, mut stream: TcpStream) {
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
        } else if let Some(resp) = app.route(&request) {
            resp
        } else {
            default_response
        }
    };
    if let Err(e) = stream.write_all(response.as_string().as_bytes()) {
        println!("Fail to response: {e:?}");
    }
}
