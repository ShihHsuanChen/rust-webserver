use std::collections::HashMap;
use super::{Response, MakeContent};


pub fn make_text_response(status_code: usize, content: String) -> Result<Response<TextContent>,String> {
    Ok(Response::<TextContent>::new(
        status_code,
        HashMap::new(),
        TextContent(content),
    )?)
}


pub struct HtmlContent (pub String);

impl MakeContent for HtmlContent {
    fn content_type(&self) -> &str {
        "text/html"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct TextContent (pub String);

impl MakeContent for TextContent {
    fn content_type(&self) -> &str {
        "text/plain"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct CssContent (pub String);

impl MakeContent for CssContent {
    fn content_type(&self) -> &str {
        "text/css"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct XmlContent (pub String);

impl MakeContent for XmlContent {
    fn content_type(&self) -> &str {
        "application/xml"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct JavascriptContent (pub String);

impl MakeContent for JavascriptContent {
    fn content_type(&self) -> &str {
        "application/javascript"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}
