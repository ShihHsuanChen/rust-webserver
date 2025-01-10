use std::{collections::HashMap, path::Path};

use super::http;

mod template;
pub use template::Template;


pub fn make_response(status_code: u32, content: String) -> Response {
    Response {
        status_code,
        headers: HashMap::new(),
        body: content,
    }
}


pub struct Response {
    status_code: u32,
    headers: HashMap<String, String>, // TODO: 
    body: String,
}

// TODO: return bytes for non-text response?
impl Response {
    pub fn as_string(&self) -> String {
        // TODO: headers
        format!(
            "{} {}\r\nContent-Length: {}\r\n\r\n{}",
            http::PROTOCOL::HTTP_1_1,
            match http::get_status_from_code(self.status_code) {
                Ok(v) => v.code,
                Err(_) => self.status_code,
            }, // TODO: handle unknown 
            self.body.len(),
            self.body,
        )
    }
}
