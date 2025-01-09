use std;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use super::http;


pub struct Request<'a> {
    pub protocal: http::Protocol<'a>,
    pub method: http::Method<'a>,
    pub path: String,
}

impl Request<'_> {
    pub fn from_stream(stream: &TcpStream) -> Result<Self, String> {
        let buf_reader = BufReader::new(stream);
        let mut request_lines = buf_reader.lines(); // NOTE: buf_reader.lines() call needs prelude::*
        let request_first_line: String;

        match request_lines.next() {
            Some(v) => match v {
                Ok(_v) => request_first_line = _v,
                Err(_) => return Err(String::from("IO Error when reading stream"))
            },
            None => return Err(String::from("Bad request"))
        }
        // first line of the request: <method> <path> <protocal>
        let mut sp = request_first_line.split(" ");
        let (method, path, protocal) = (
            // http::METHOD::GET, "/".to_string(), http::PROTOCAL::HTTP_1_1,
            http::get_method_from_str(sp.next().unwrap()).unwrap(),
            sp.next().unwrap().trim().to_string(),
            http::get_protocal_by_str(sp.next().unwrap()).unwrap(),
        );

        // let mut i = 1;
        // loop {
        //     i += 1;
        //     let request_second_line: String;
        //     match request_lines.next() {
        //         Some(v) => match v {
        //             Ok(_v) => {
        //                 if _v.is_empty() {
        //                     break;
        //                 } else {
        //                     request_second_line = _v;
        //                 }
        //             }
        //             Err(e) => { println!(">>> Error: {e:?}"); break; return Err(String::from("IO Error when reading stream")) },
        //         },
        //         None => { println!(">>> Stop"); break; }, // return Err(String::from("Bad request"))
        //     }
        //     println!("{i} {}", request_second_line);
        // }
        Ok(Request { protocal, method, path })
    }
}


impl std::fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request(\r\n  protocal: {},\r\n  path: {},\r\n  method: {}\r\n)", self.protocal, self.path, self.method)
    }
}

