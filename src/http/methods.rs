use std;

pub struct Method<'a>(pub &'a str);


impl<'a> std::cmp::PartialEq for Method<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> std::fmt::Display for Method<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_uppercase())
    }
}

pub mod METHOD {
    pub const GET:     super::Method<'_> = super::Method("GET");
    pub const POST:    super::Method<'_> = super::Method("POST");
    pub const PUT:     super::Method<'_> = super::Method("PUT");
    pub const PATCH:   super::Method<'_> = super::Method("PATCH");
    pub const DELETE:  super::Method<'_> = super::Method("DELETE");
    pub const HEAD:    super::Method<'_> = super::Method("HEAD");
    pub const OPTIONS: super::Method<'_> = super::Method("OPTIONS");
    pub const CONNECT: super::Method<'_> = super::Method("CONNECT");
    pub const TRACE:   super::Method<'_> = super::Method("TRACE");
}

pub fn get_method_from_str(method_str: &str) -> Result<Method<'static>, String> {
    for v in [
        METHOD::GET,
        METHOD::POST,
        METHOD::PUT,
        METHOD::PATCH,
        METHOD::DELETE,
        METHOD::HEAD,
        METHOD::OPTIONS,
        METHOD::CONNECT,
        METHOD::TRACE,
    ] {
        if method_str == v.0 {
            return Ok(v)
        }
    }
    Err(format!("Unknown method {method_str}."))
}
