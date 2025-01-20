use std::collections::HashMap;

use crate::request::content_type::Json;

use super::{Response, MakeContent};


pub fn make_json_response(status_code: usize, content: Json) -> Result<Response<JsonContent>,String> {
    Ok(Response::<JsonContent>::new(
        status_code,
        HashMap::new(),
        JsonContent(content),
    )?)
}


pub struct JsonContent (pub Json);

impl MakeContent for JsonContent {
    fn content_type(&self) -> &str {
        "application/json"
    }
    fn into_bytes(&self) -> Vec<u8> {
        self.0.dump().into_bytes()
    }
}
