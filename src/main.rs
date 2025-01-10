use std;
use std::io::Write;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

use webserver::thread_pool::ThreadPool;
use webserver::http::{METHOD, PROTOCOL};
use webserver::response::{
    make_response,
    template::Template,
};
use webserver::request::Request;


const TEMPLATE: Template<'_> = Template { root: "templates" };


fn handle_connection(mut stream: TcpStream) {
    println!("Connection established");
    let request = match Request::from_stream(&stream) {
        Ok(v) => v,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };
    println!("{}", request);

    let args = HashMap::new();

    let response = {
        if request.protocol != PROTOCOL::HTTP_1_1 {
            make_response(404, "Not found")
        } else if request.method != METHOD::GET {
            make_response(404, "Not found")
        } else if let Ok(v) = TEMPLATE.make_response(200, &request.path[1..], &args) { // TODO: Path operation
            v
        } else if let Ok(v) = TEMPLATE.make_response(404, "404.html", &args) {
            v
        } else {
            make_response(404, "Not found")
        }
    };
    if let Err(e) = stream.write_all(response.as_bytes()) {
        println!("Fail to response: {e:?}");
    }
}


fn main() {
    let pool = ThreadPool::new(4);
    let ip_port = String::from("127.0.0.1:7878");
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
            handle_connection(stream);
        });
    }
}
