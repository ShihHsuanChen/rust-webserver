use std::collections::HashMap;

use crate::json::JsonValue;


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

pub enum ContentType {
    Text(String),
    File(File),
    Json(JsonValue),
    Form(HashMap<String,ContentType>),
    Binary(Binary),
    None,
}
