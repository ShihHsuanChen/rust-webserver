use std::collections::HashMap;

use crate::json::{JsonValue, dump as json_dump};

use super::{Response, MakeContent};


pub fn make_json_response(status_code: usize, content: JsonValue) -> Result<Response<MakeJsonContent>,String> {
    Ok(Response::<MakeJsonContent>::new(
        status_code,
        HashMap::new(),
        MakeJsonContent(content),
    )?)
}


pub struct MakeJsonContent (pub JsonValue);

impl MakeContent for MakeJsonContent {
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
