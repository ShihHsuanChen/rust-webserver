use std::collections::HashMap;

use crate::http;

use super::{Response, MakeContent, MakeContentData};


pub fn make_text_response(status_code: usize, content: String) -> Result<Response<MakeTextContent>,String> {
    Ok(Response::<MakeTextContent>::new(
        status_code,
        HashMap::new(),
        MakeTextContent(content),
    )?)
}


pub struct MakeTextLikeContent {
    pub content: String,
    pub content_type: String,
}

impl MakeContent for MakeTextLikeContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: self.content.clone(),
            content_length: self.content.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }
}


pub struct MakeHtmlContent (pub String);

impl MakeContent for MakeHtmlContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("text/html"),
            content_length: self.0.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeTextContent (pub String);

impl MakeContent for MakeTextContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("text/plain"),
            content_length: self.0.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeCssContent (pub String);

impl MakeContent for MakeCssContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("text/css"),
            content_length: self.0.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeXmlContent (pub String);

impl MakeContent for MakeXmlContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("application/xml"),
            content_length: self.0.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}


pub struct MakeJavascriptContent (pub String);

impl MakeContent for MakeJavascriptContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("application/javascript"),
            content_length: self.0.len(),
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}
