use std::collections::HashMap;


pub struct File {
    pub filename: String,
    pub filename_encoded: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
}

pub struct Binary {
    pub content_type: Option<String>,
    pub content: Vec<u8>,
}

pub type Json = json::JsonValue;

pub enum ContentType {
    Text(String),
    File(File),
    Json(json::JsonValue),
    Form(HashMap<String,ContentType>),
    Binary(Binary),
    None,
}
