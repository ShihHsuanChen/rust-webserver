use std::collections::HashMap;

use super::http::{self, STATUS_SP, CRLF};

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
    pub status_code: u32,
    pub headers: HashMap<String, String>, // TODO: 
    pub body: String,
}

// TODO: return bytes for non-text response?
impl Response {
    pub fn as_string(&self) -> String {
        // TODO: headers
        // HTTP/1.1 Response:
        //   Status-Line
        //   *(( general-header
        //   | response-header
        //   | entity-header ) CRLF)
        //   CRLF
        //   [ message-body ]
        format!(
            "{}{STATUS_SP}{}{CRLF}Content-Length: {}{CRLF}{CRLF}{}",
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
