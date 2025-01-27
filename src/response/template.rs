use std;
use std::path::Path;
use std::collections::HashMap;

use regex::{Regex, Captures};

use crate::http;

use super::{Response, MakeTextLikeContent};


pub struct Template<'a> {
    pub root: &'a str,
}

fn guess_content_type(path: &str) -> String {
    let path = Path::new(path);
    (match path.extension() {
        Some(osstr) => match osstr.to_str() {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("xml") => "application/xml",
            _ => "text/plain",
        },
        None => "text/plain",
    }).to_string()
}


pub fn replace_args(content: &str, args: &HashMap<String,String>) -> String {
    let re = Regex::new(r"\{ *(\w+) *\}").unwrap();
    let empty = String::from("");
    re.replace_all(content, |caps: &Captures| {
        args.get(&caps[1]).unwrap_or(&empty.clone()).to_owned()
    }).to_string().to_owned()
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
        args: &HashMap<String, String>,
        extra_headers: http::Headers,
    ) -> Result<Response<MakeTextLikeContent>, String>
    {
        let path = Path::new(self.root).join(path);
        let path_str = match path.to_str() {
            Some(v) => v,
            None => panic!("Invalid path {path:?}"), // TODO: if path is not a valid unicode ???
        };
        let content_type = guess_content_type(path_str);
        if path.is_file() {
            match std::fs::read_to_string(path_str) {
                Ok(content) => {
                    let content = replace_args(&content, args);
                    Ok(Response::<MakeTextLikeContent>::new(
                        status_code,
                        extra_headers,
                        MakeTextLikeContent { content, content_type },
                    )?)
                },
                Err(_) => Err(format!("Fail to read file from {}", path_str)),
            }// path from repo 
        } else {
            Err(format!("{} is not a file", path_str))
        }
    }
}
