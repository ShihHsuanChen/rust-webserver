use std;
use std::collections::HashMap;
use std::io::BufReader;
use std::net::TcpStream;

use super::http;

mod parser;
pub mod content_type;

use content_type::ContentType;
use parser::{
    parse_readout,
    parse_readout_body__text,
    parse_readout_body__x_www_form_urlencoded,
    parse_readout_body__multipart,
    parse_readout_body__binary,
};


pub struct Request<'a> {
    pub protocol: http::Protocol<'a>,
    pub method: http::Method<'a>,
    pub path: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub query: HashMap<String, String>,
    pub fragment: Option<String>,
    pub headers: HashMap<String,String>,
    pub body: ContentType,
}


impl Request<'_> {
    pub fn from_stream(stream: &TcpStream) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(stream);
        let res = match parse_readout(&mut buf_reader) {
            Ok(v) => v, Err(e) => return Err(e),
        };
        let protocol = res.protocol.unwrap();
        let method = res.method.unwrap();
        let url = res.url.unwrap();
        let path = url.path().to_string();
        let username = Some(url.username().to_owned());
        let password = match url.password() {
            Some(v) => Some(v.to_owned()), None => None,
        };
        let query = res.query;
        let fragment = match url.fragment() {
            Some(v) => Some(v.to_owned()), None => None,
        };
        let headers = res.headers.unwrap();
        let body = res.body.unwrap();
        let body_boundary = res.boundary;
        // content-type
        let none = String::from("none");
        let content_type = &headers.get("Content-Type").unwrap_or(&none)[..];
        let content: ContentType = match content_type {
            "multipart/form-data" => {
                if body_boundary.is_none() {
                    return Err(String::from("Multipart/form-data boundary not found"));
                }
                let boundary = body_boundary.unwrap();
                match parse_readout_body__multipart(&body, &boundary) {
                    Ok(res) => Some(res),
                    Err(e) => return Err(e),
                }
            },
            "application/x-www-form-urlencoded" => {
                match parse_readout_body__x_www_form_urlencoded(&body) {
                    Ok(res) => Some(res),
                    Err(e) => return Err(e),
                }
            },
            "application/json" | 
            "application/javascript" | "text/javascript" |
            "application/css" | "text/css" |
            "text/html" | "text/plain" |
            "application/xml" | "text/xml" => {
                match parse_readout_body__text(&body, content_type) {
                    Ok(res) => Some(res),
                    Err(e) => return Err(e),
                }
            },
            "none" => {
                None
            },
            _ => {
                match parse_readout_body__binary(&body, content_type) {
                    Ok(res) => Some(res),
                    Err(e) => return Err(e),
                }
            }
        };
        Ok(Request {
            protocol,
            method,
            path,
            username,
            password,
            query,
            fragment,
            headers,
            body: content,
        })
    }
}


impl std::fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Request(\r\n\
                protocol: {},\r\n    \
                method: {},\r\n    \
                path: {},\r\n    \
                query: {:?},\r\n    \
                fragment: {:?},\r\n    \
                headers: {:?},\r\n\
            )",
            self.protocol,
            self.method,
            self.path,
            self.query,
            self.fragment,
            self.headers,
        )
    }
}

