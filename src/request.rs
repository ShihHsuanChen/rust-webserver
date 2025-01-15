use std;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use json;
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
    pub headers: HashMap<String,String>,
    pub body: Vec<u8>,
}


fn parse_readout_first_line(line: String) -> Result<(http::Protocol<'static>, http::Method<'static>, Url), String> {
    let mut sp = line.split(" ");
    let parse_err = format!("Unknown request format {line}");

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
    Ok((protocol, method, url))
}

fn parse_readout_header_lines(line: String) -> Result<(String,String), String> {
    match line.split_once(": ") {
        Some(kv) => {
            let (k, v) = kv;
            Ok((String::from(k), String::from(v)))
        },
        None => Err(String::from("Fail to parse request header: {line}")),
    }
}

/// Content-Types:
/// - text/plain
/// - text/html
/// - application/xml
/// - application/json
/// - application/javascript
pub fn parse_readout_body__raw_text(buf: &Vec<u8>) -> Result<String,String> {
    match std::str::from_utf8(&buf) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err(String::from("Fail to convert binary to string."))
    }

}

/// Content-Types:
/// - application/javascript
pub fn parse_readout_body__json(buf: &Vec<u8>) -> Result<json::JsonValue,String> {
    // buf should be able to convert to utf-8 string
    match std::str::from_utf8(&buf) {
        Ok(json_str) => {
            println!("'{json_str}'");
            match json::parse(json_str) {
                Ok(v) => Ok(v),
                Err(e) => {
                    println!("{e:?}");
                    Err(String::from("Not a valid json string"))
                }
            }
        },
        Err(_) => Err(String::from("Fail to convert binary to string."))
    }
}

/// Content-Types:
/// - application/x-www-form-urlencoded
pub fn parse_readout_body__x_www_form_urlencoded(buf: &Vec<u8>) -> Result<HashMap<String,String>,String> {
    match std::str::from_utf8(&buf) {
        Ok(s) => {
            Ok(parse_urlencoded(s))
        },
        Err(_) => Err(String::from("Fail to convert binary to string."))
    }
}

struct ParseResult {
    protocol: Option<http::Protocol<'static>>,
    method: Option<http::Method<'static>>,
    url: Option<Url>,
    headers: Option<HashMap::<String,String>>,
    body: Option<Vec<u8>>,
}


pub fn parse_urlencoded(s: &str) -> HashMap<String,String> {
    let mut tmp = HashMap::<String,String>::new();
    let mut pairs = url::form_urlencoded::parse(s.as_bytes());
    while let Some(pair) = pairs.next() {
        tmp.insert(
            pair.0.into_owned().to_string(),
            pair.1.into_owned().to_string(),
        );
    }
    tmp
}


fn parse_readout(buf_reader: &mut BufReader<&TcpStream>) -> Result<ParseResult, String> {
    let mut register: Vec<u8> = vec![];
    let mut last: Option<u8> = None;
    let mut iter = buf_reader.bytes();
    let mut end_of_header = false;
    let mut cl: u32 = 0;
    let mut iline: u32 = 0;
    // data
    let mut headers = HashMap::<String,String>::new();
    let mut result = ParseResult {
        method: None,
        protocol: None,
        url: None,
        headers: None,
        body: None,
    };
    while let Some(byte) = iter.next() {
        // println!("{end_of_header}");
        match byte {
            Ok(v) => {
                if let Some(_v) = last {
                    if _v == 13 && v == 10 { // append line
                        match std::str::from_utf8(&register) {
                            Ok(line) => {
                                let _line = line.to_string();
                                if _line == "" {
                                    // blank line as the separator of header and body
                                    end_of_header = true;
                                    if cl <= 1 {
                                        break;
                                    }
                                    cl -= 1;
                                } else {
                                    if iline == 0 {
                                        match parse_readout_first_line(_line) {
                                            Ok(v) => {
                                                result.protocol = Some(v.0);
                                                result.method = Some(v.1);
                                                result.url = Some(v.2);
                                            },
                                            Err(e) => return Err(e),
                                        }
                                    } else {
                                        match parse_readout_header_lines(_line) {
                                            Ok(kv) => {
                                                let (k, v) = kv;
                                                if k == "Content-Length" {
                                                    cl = v.parse().unwrap();
                                                }
                                                headers.insert(k, v);
                                            },
                                            Err(e) => return Err(e),
                                        }
                                    }
                                    iline += 1;
                                }
                                register.clear();
                                last = None;
                            },
                            Err(_) => {
                                println!("Cannot parse {register:?}");
                                register.push(_v);
                                last = Some(v);
                                continue;
                            },
                        }
                    } else {
                        register.push(_v);
                        last = Some(v);
                    }
                } else if end_of_header { // body
                    register.push(v);
                    if cl > 0 {
                        cl -= 1;
                    } else {
                        break;
                    }
                } else { // first or new line
                    last = Some(v);
                }
            },
            Err(_) => break,
        }
    }
    result.headers = Some(headers);
    result.body = Some(register);
    Ok(result)
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
        let query = parse_urlencoded(url.query().unwrap_or(""));
        let fragment = match url.fragment() {
            Some(v) => Some(v.to_owned()), None => None,
        };
        let headers = res.headers.unwrap();
        let body = res.body.unwrap();
        println!("{headers:?}");
        Ok(Request {
            protocol,
            method,
            path,
            username,
            password,
            query,
            fragment,
            headers,
            body,
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

