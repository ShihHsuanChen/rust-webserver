use super::http;

pub mod template;


pub fn make_response(status_code: u32, contents: &str) -> String {
    format!(
        "{} {}\r\nContent-Length: {}\r\n\r\n{}",
        http::PROTOCOL::HTTP_1_1,
        match http::get_status_from_code(status_code) {
            Ok(v) => v.code,
            Err(_) => status_code,
        }, // TODO: handle unknown 
        contents.len(),
        contents,
    )
}
