use std::collections::HashMap;

use crate::json::{JsonValue, dump as json_dump};
use crate::http;

use super::{Response, MakeContent, MakeContentData};


pub fn make_json_response(status_code: usize, content: JsonValue) -> Result<Response<MakeJsonContent>,String> {
    Ok(Response::<MakeJsonContent>::new(
        status_code,
        HashMap::new(),
        MakeJsonContent(content),
    )?)
}


pub struct MakeJsonContent (pub JsonValue);

impl MakeContent for MakeJsonContent {
    fn data(&self) -> MakeContentData {
        MakeContentData {
            content_type_headers: http::Headers::new(),
            content_type: String::from("application/json"),
            content_length: match json_dump(&self.0) {
                Ok(v) => v.len(),
                _ => 0,
            }
        }
    }
    fn into_bytes(&self) -> Vec<u8> {
        if let Ok(v) = json_dump(&self.0) {
            v.into_bytes()
        } else {
            vec![]
        }
    }
}
