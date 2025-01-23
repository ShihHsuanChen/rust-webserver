use std::collections::HashMap;

use crate::json::{JsonValue, dump as json_dump};

use super::{Response, MakeContent};


pub fn make_json_response(status_code: usize, content: JsonValue) -> Result<Response<JsonContent>,String> {
    Ok(Response::<JsonContent>::new(
        status_code,
        HashMap::new(),
        JsonContent(content),
    )?)
}


pub struct JsonContent (pub JsonValue);

impl MakeContent for JsonContent {
    fn content_length(&self) -> usize {
        if let Ok(v) = json_dump(&self.0) {
            v.len()
        } else {
            0
        }
    }
    fn content_type(&self) -> &str {
        "application/json"
    }
    fn into_bytes(&self) -> Vec<u8> {
        if let Ok(v) = json_dump(&self.0) {
            v.into_bytes()
        } else {
            vec![]
        }
    }
}
