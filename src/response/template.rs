use std;
use std::path::Path;
use std::collections::HashMap;

use super::{Response, HtmlContent};


pub struct Template<'a> {
    pub root: &'a str,
}

impl<'a> Template<'a> {
    pub fn new(root: &'a str) -> Result<Self, String> {
        let path = Path::new(root);
        if path.is_dir() {
            Ok(Template { root: root })
        } else {
            Err(String::from("Given path is not a valid directory"))
        }
    }

    pub fn make_response(
        &self,
        status_code: usize,
        path: &str,
        args: &HashMap<String, String>
    ) -> Result<Response<HtmlContent>, String>
    {
        let path = Path::new(self.root).join(path);
        let path_str = match path.to_str() {
            Some(v) => v,
            None => "/", // TODO: if path is not a valid unicode ???
        };
        if path.is_file() {
            match std::fs::read_to_string(path_str) {
                Ok(content) => {
                    Ok(Response::<HtmlContent>::new(
                        status_code,
                        HashMap::new(),
                        HtmlContent(content),
                    )?)
                },
                Err(_) => Err(format!("Fail to read file from {}", path_str)),
            }// path from repo 
        } else {
            Err(format!("{} is not a file", path_str))
        }
    }
}
