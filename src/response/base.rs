use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;

use crate::http::{self, STATUS_SP, HEADER_SP, HEADER_META_SP, CRLF};


pub trait MakeResponse {
    // HTTP Response:
    //   Status-Line
    //   *(( general-header
    //   | response-header
    //   | entity-header ) CRLF)
    //   CRLF
    //   [ message-body ]
    // where Status-Line is:
    //   HTTP-Version SP Status-Code SP Reason-Phrase CRLF

    // attributes
    fn protocol(&self) -> &http::Protocol<'static>;
    fn status(&self) -> &http::Status<'static>;
    fn headers(&self) -> &HashMap<String,(String,Vec<String>)>; // key,value,metadata
    fn content_length(&self) -> usize {
        // count directly by default
        let mut cnt: usize = 0;
        for bytes in self.messege_body() {
            cnt += bytes.len();
        }
        cnt
    }
    fn messege_body(&self) -> impl Iterator<Item=Vec<u8>>;

    // derived attributes
    fn status_line(&self) -> String {
        // HTTP-Version SP Status-Code SP Reason-Phrase CRLF
        format!("{}{STATUS_SP}{}{STATUS_SP}{}",
            self.protocol(),
            self.status().code,
            self.status().name,
        )
    }

    fn header_lines(&self) -> String {
        let mut s = String::from("");
        for (key, (value, metadata)) in self.headers().iter() {
            s.push_str(&format!("{key}{HEADER_SP} "));
            if key == "Content-length" {
                s.push_str(&self.content_length().to_string());
            } else {
                s.push_str(value);
            }
            for item in metadata.iter() {
                s.push_str(&format!("{HEADER_META_SP} {item}"));
            }
            s.push_str(CRLF)
        }
        s
    }

    // methods
    fn print_response(&self) {
        print!("{}", self.status_line());
        print!("{}", self.header_lines());
        print!("{CRLF}");
        for bytes in self.messege_body() {
            if let Ok(s) = std::str::from_utf8(&bytes) {
                print!("{s}");
            } else {
                print!("{bytes:?}");
            }
        }
    }

    fn write(&self, mut stream: TcpStream) -> std::io::Result<usize> {
        let mut nbytes: usize = 0;
        nbytes += stream.write(self.status_line().as_bytes())?;
        nbytes += stream.write(self.header_lines().as_bytes())?;
        nbytes += stream.write(CRLF.as_bytes())?;
        stream.flush()?;
        for bytes in self.messege_body() {
            nbytes += stream.write(&bytes)?;
            stream.flush()?;
        }
        Ok(nbytes)
    }
}

pub trait MakeContent {
    fn content_type(&self) -> &str;
    fn into_bytes(&self) -> Vec<u8>;
}


pub struct Response<T: MakeContent> {
    protocol: http::Protocol<'static>,
    status: http::Status<'static>,
    headers: HashMap<String,(String,Vec<String>)>, // key,value,metadata
    content: T
}

impl<T: MakeContent> Response<T> {
    pub fn new(
        status_code: usize,
        headers: HashMap<String,(String,Vec<String>)>,
        content: T,
    ) -> Result<Self,String> {
        let status = http::get_status_from_code(status_code)?;
        Ok(Response::<T> {
            protocol: http::PROTOCOL::HTTP_1_1,
            status,
            headers,
            content,
        })


    }
}

impl<T: MakeContent> MakeResponse for Response<T> {
    fn protocol(&self) -> &http::Protocol<'static> {
        &self.protocol
    }
    fn status(&self) -> &http::Status<'static> {
        &self.status
    }
    fn headers(&self) -> &HashMap<String,(String,Vec<String>)> {
        &self.headers
    }
    fn messege_body(&self) -> impl Iterator<Item=Vec<u8>> {
        let vec: Vec<Vec<u8>> = vec![self.content.into_bytes()];
        vec.into_iter()
    }
}
