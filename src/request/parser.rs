use std;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use json;
use url::{Url, form_urlencoded};

use super::http;
use super::content_type::{File, Binary, ContentType};


struct HeaderLine {
    key: String,
    value: String,
    metadata: HashMap<String,String>,
}

pub struct ParseResult {
    pub protocol: Option<http::Protocol<'static>>,
    pub method: Option<http::Method<'static>>,
    pub url: Option<Url>,
    pub query: HashMap::<String,String>,
    pub headers: Option<HashMap::<String,String>>,
    pub body: Option<Vec<u8>>,
    pub boundary: Option<String>,
}


pub fn parse_readout(buf_reader: &mut BufReader<&TcpStream>) -> Result<ParseResult, String> {
    // HTTP/1.1 Request:
    //   Status-Line
    //   *(( general-header
    //   | request-header
    //   | entity-header ) CRLF)
    //   CRLF
    //   [ message-body ]
    let mut register: Vec<u8> = vec![];
    let mut last: Option<u8> = None;
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
        boundary: None,
        query: HashMap::<String,String>::new(),
    };
    for byte in buf_reader.bytes() {
        let v = byte.unwrap();
        if let Some(_v) = last {
            // not linesep, append to register
            if !(_v == 13 && v == 10) {
                register.push(_v);
                last = Some(v);
                continue;
            }
            // meet linesep line
            if let Ok(line) = std::str::from_utf8(&register) {
                let _line = line.to_string();
                if _line == "" {
                    // blank line as the separator of header and body
                    end_of_header = true;
                    if cl <= 1 { break; }
                    cl -= 1;
                } else if iline == 0 {
                    match parse_readout_status_line(_line) {
                        Ok(v) => {
                            result.protocol = Some(v.0);
                            result.method = Some(v.1);
                            result.query = parse_urlencoded(v.2.query().unwrap_or(""));
                            result.url = Some(v.2);
                        },
                        Err(e) => return Err(e),
                    }
                } else {
                    match parse_readout_header_line(&_line) {
                        Ok(h) => {
                            let (hk, hv) = (h.key, h.value);
                            if hk == "Content-Length" {
                                cl = hv.parse().unwrap();
                                headers.insert(hk, hv);
                            } else if hk == "Content-Type" {
                                if let Some(b) = h.metadata.get("boundary") {
                                    result.boundary = Some(b.to_string());
                                } else {
                                    headers.insert(hk, hv);
                                }
                            } else {
                                headers.insert(hk, hv);
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
                iline += 1;
                register.clear();
                last = None;
            } else {
                println!("Cannot parse {register:?}");
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
    }
    result.headers = Some(headers);
    result.body = Some(register);
    Ok(result)
}


pub fn parse_urlencoded(s: &str) -> HashMap<String,String> {
    let mut tmp = HashMap::<String,String>::new();
    let mut pairs = form_urlencoded::parse(s.as_bytes());
    while let Some(pair) = pairs.next() {
        tmp.insert(
            pair.0.into_owned().to_string(),
            pair.1.into_owned().to_string(),
        );
    }
    tmp
}


fn parse_readout_status_line(line: String) -> Result<(http::Protocol<'static>, http::Method<'static>, Url), String> {
    //  Status-Line:
    //  HTTP-Version SP Status-Code SP Reason-Phrase CRLF
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


fn parse_readout_header_line(line: &str) -> Result<HeaderLine, String> {
    // Example line:
    // Content-Disposition: form-data; name=\"b\"
    // Content-Disposition: form-data; name=\"b\"; filename=\"abcabcabc.docx\"
    let segs: Vec<_> = line.split("; ").collect();

    if let Some(kv) = segs[0].split_once(": ") {
        let (k, v) = kv;
        let mut metadata = HashMap::<String,String>::new();
        if segs.len() > 1 {
            for seg in &segs[1..] {
                if let Some(m) = seg.split_once("=") {
                    let (mk, mut mv) = m;
                    // unwrap \" from value
                    if mv.starts_with("\"") { mv = &mv[1..] }
                    if mv.ends_with("\"") { mv = &mv[..mv.len()-1] }
                    metadata.insert(mk.to_string(), mv.to_string());
                }
            }
        }
        Ok(HeaderLine {
            key: k.to_string(),
            value: v.to_string(),
            metadata,
        })
    } else {
        return Err(String::from("Fail to parse request header: {line}"));
    }
}


/// Content-Types:
/// - text/plain
/// - text/html
/// - text/css
/// - application/xml
/// - application/javascript
/// - ...
pub fn parse_readout_body__text(buf: &Vec<u8>) -> Result<String,String> {
    match std::str::from_utf8(&buf) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err(String::from("Fail to convert binary to string."))
    }

}


/// Content-Types:
/// - application/json
pub fn parse_readout_body__json(buf: &Vec<u8>) -> Result<json::JsonValue,String> {
    // buf should be able to convert to utf-8 string
    match std::str::from_utf8(&buf) {
        Ok(json_str) => {
            match json::parse(json_str) {
                Ok(v) => Ok(v),
                Err(e) => {
                    Err(String::from("Not a valid json string"))
                }
            }
        },
        Err(_) => Err(String::from("Fail to convert binary to string."))
    }
}


/// Content-Types:
/// - application/x-www-form-urlencoded
pub fn parse_readout_body__x_www_form_urlencoded(buf: &Vec<u8>) -> Result<HashMap<String,ContentType>,String> {
    if let Ok(s) = std::str::from_utf8(&buf) {
        let mut res = HashMap::<String,ContentType>::new();
        for (k,v) in parse_urlencoded(s) {
            res.insert(k, ContentType::Text(v));
        }
        Ok(res)
    } else {
        Err(String::from("Fail to convert binary to string."))
    }
}


// TODO: better way?
/// Content-Types:
/// - multipart/form-data
pub fn parse_readout_body__multipart(buf: &Vec<u8>, boundary: &str) -> Result<HashMap<String,ContentType>,String> {
    // boundary=--------------------------896280056578890900126354
    /* Example
    ----------------------------896280056578890900126354
    Content-Disposition: form-data; name="a"

    1
    ----------------------------896280056578890900126354
    Content-Disposition: form-data; name="b"

    asdasd
    ----------------------------896280056578890900126354--\r\n
    */

    let extended_boundary = format!("--{boundary}");
    let sep: &[u8] = &extended_boundary.as_bytes();
    let l = sep.len();
    let mut blocks: Vec<(Vec<String>,Vec<u8>)> = vec![];
    let mut register: Vec<u8> = vec![];
    let mut writting_header = false;
    let mut writting_content = false;
    let mut last: Option<u8> = None;
    let mut header_lines: Vec<String> = vec![];

    for v in buf.iter() {
        if let Some(_v) = last {
            // not linesep, append to register
            if !(_v == 13 && *v == 10) {
                register.push(_v);
                last = Some(*v);
                continue;
            }
            // meet linesep
            let n = register.len();
            if writting_content && n > l+2
                && (&register[n-l..n] == sep || &register[n-l-2..n-2] == sep) {
                // register: ...\r\n-------xxxxxxxxx
                let content = &register[..n-l-2].to_vec();
                blocks.push((header_lines.clone(), content.clone()));
                header_lines.clear();
                register.clear();
                last = None;
                writting_header = true;
                writting_content = false;
            } else if &register == sep { // meet boundary
                // register: ...-------xxxxxxxxx
                writting_header = true;
                register.clear();
                last = None;
            } else if writting_header {
                if let Ok(line) = std::str::from_utf8(&register) {
                    let _line = line.to_string();
                    if _line == "" {
                        // blank line as the segarator of header and content
                        writting_header = false;
                        writting_content = true;
                    } else {
                        header_lines.push(line.to_string());
                    }
                    last = None;
                } else {
                    return Err(String::from("Fail to parse form-data/multipart block header."));
                }
                register.clear();
            } else {
                // writting content but not meet boundary yet
                register.push(_v);
                last = Some(*v);
            }
        } else {
            last = Some(*v);
        }
    }
    let mut res = HashMap::<String,ContentType>::new();
    for (block,content) in blocks.iter() {
        let mut headers = HashMap::<String, HeaderLine>::new();
        for line in block.iter() {
            if let Ok(header) = parse_readout_header_line(line) {
                headers.insert(header.key.to_string(), header);
            }
        }
        if let Some(header) = headers.get("content-Disposition") {
            if let Some(key) = header.metadata.get("name") {
                if let Some(filename) = header.metadata.get("filename") {
                    // File
                    let v = ContentType::File(File {
                        filename: filename.to_string(),
                        filename_encoded: {
                            header.metadata.get("filename*")
                            .unwrap_or(filename)
                            .to_string()
                        },
                        content_type: {
                            match header.metadata.get("Content-Type") {
                                Some(v) => Some(v.to_string()),
                                None => None,
                            }
                        },
                        content: content.to_vec(),
                    });
                    res.insert(key.to_string(), v);
                } else if let Ok(s) = std::str::from_utf8(content) {
                    let v = ContentType::Text(s.to_string());
                    res.insert(key.to_string(), v);
                } else {
                    println!("Fail to parse mutipart/form-data content of '{key}'");
                }
            } else {
                // No name
            }
        } else {
            // No Content-Disposition
        }
    }
    Ok(res)
}


/// Content-Types:
/// - application/x-www-form-urlencoded
pub fn parse_readout_body__binary(buf: &Vec<u8>, content_type: &str) -> Result<ContentType,String> {
    Ok(ContentType::Binary( Binary {
        content_type: Some(content_type.to_string()),
        content: buf.to_vec(),
    }))
}
