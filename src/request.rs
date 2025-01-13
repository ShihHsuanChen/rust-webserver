use std;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use url::Url;

use super::http;


pub struct Request<'a> {
    pub protocol: http::Protocol<'a>,
    pub method: http::Method<'a>,
    pub path: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub query: HashMap<String, String>,
    pub fragment: Option<String>,
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
        // first line of the request: <method> <path> <protocol>
        let mut sp = request_first_line.split(" ");
        let parse_err = format!("Unknown request format {request_first_line}");

        // method
        let method_str = match sp.next() {
            Some(v) => v, None => return Err(parse_err),
        };
        let method = match http::get_method_from_str(method_str) {
            Ok(v) => v, Err(e) => return Err(e),
        };
        let url = if let Some(v) = sp.next() {
            // TODO: fake host?
            if let Ok(_v) = Url::parse(&format!("http://localhost{v}")) {
                _v
            } else {
                return Err(parse_err);
            }
        } else {
            return Err(parse_err)
        };

        // protocol
        let protocol_str = match sp.next() {
            Some(v) => v, None => return Err(parse_err),
        };
        let protocol = match http::get_protocol_from_str(protocol_str) {
            Ok(v) => v, Err(e) => return Err(e),
        };

        // TODO: Cannot get the request Body. why??
        // let mut i = 1;
        // loop {
        //     i += 1;
        //     let request_second_line: String;
        //     match request_lines.next() {
        //         Some(v) => match v {
        //             Ok(_v) => {
        //                 if _v.is_empty() {
        //                     // TODO: this will truncate the request body
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
        Ok(Request {
            protocol,
            method,
            path: url.path().to_string(),
            username: Some(url.username().to_owned()),
            password: match url.password() {
                Some(v) => Some(v.to_owned()), None => None,
            },
            // query: HashMap::<String,String>::new(),
            query: {
                let mut tmp = HashMap::<String,String>::new();
                let mut pairs = url.query_pairs();
                while let Some(pair) = pairs.next() {
                    tmp.insert(
                        pair.0.into_owned().to_string(),
                        pair.1.into_owned().to_string(),
                    );
                }
                tmp
            },
            fragment: match url.fragment() {
                Some(v) => Some(v.to_owned()), None => None,
            }
        })
    }
}


impl std::fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request(\r\n  protocol: {},\r\n  path: {},\r\n  method: {}\r\n)", self.protocol, self.path, self.method)
    }
}

