use std::collections::HashMap;
use super::{Response, MakeContent};


pub fn make_text_response(status_code: usize, content: String) -> Result<Response<MakeTextContent>,String> {
    Ok(Response::<MakeTextContent>::new(
        status_code,
        HashMap::new(),
        MakeTextContent(content),
    )?)
}


pub struct MakeHtmlContent (pub String);

impl MakeContent for MakeHtmlContent {
    fn content_length(&self) -> usize {
        self.0.len()
    }
    fn content_type(&self) -> &str {
        "text/html"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeTextContent (pub String);

impl MakeContent for MakeTextContent {
    fn content_length(&self) -> usize {
        self.0.len()
    }
    fn content_type(&self) -> &str {
        "text/plain"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeCssContent (pub String);

impl MakeContent for MakeCssContent {
    fn content_length(&self) -> usize {
        self.0.len()
    }
    fn content_type(&self) -> &str {
        "text/css"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeXmlContent (pub String);

impl MakeContent for MakeXmlContent {
    fn content_length(&self) -> usize {
        self.0.len()
    }
    fn content_type(&self) -> &str {
        "application/xml"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeJavascriptContent (pub String);

impl MakeContent for MakeJavascriptContent {
    fn content_length(&self) -> usize {
        self.0.len()
    }
    fn content_type(&self) -> &str {
        "application/javascript"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}
