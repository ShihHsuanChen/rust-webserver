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
    pub const GET: super::Method<'_>    = super::Method("GET");
    pub const POST: super::Method<'_>   = super::Method("POST");
    pub const PUT: super::Method<'_>    = super::Method("PUT");
    pub const PATCH: super::Method<'_>  = super::Method("PATCH");
    pub const DELETE: super::Method<'_> = super::Method("DELETE");
    pub const HEAD: super::Method<'_>   = super::Method("HEAD");
    pub const OPTION: super::Method<'_> = super::Method("OPTION");
}

pub fn get_method_from_str(method_str: &str) -> Result<Method<'static>, String> {
    for v in [
        METHOD::GET,
        METHOD::POST,
        METHOD::PUT,
        METHOD::PATCH,
        METHOD::DELETE,
        METHOD::HEAD,
        METHOD::OPTION,
    ] {
        if method_str == v.0 {
            return Ok(v)
        }
    }
    Err(format!("Unknown method {method_str}."))
}

pub struct Protocol<'a> {
    pub protocol: &'a str,
    pub version: &'a str,
}

impl<'a> std::fmt::Display for Protocol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.protocol.to_uppercase(), self.version)
    }
}

impl<'a> std::cmp::PartialEq for Protocol<'a> { fn eq(&self, other: &Self) -> bool {
    self.protocol == other.protocol && self.version == other.version
}
}

pub mod PROTOCOL {
    pub const HTTP_1_0: super::Protocol<'_> = super::Protocol { protocol: "HTTP", version: "1" };
    pub const HTTP_1_1: super::Protocol<'_> = super::Protocol { protocol: "HTTP", version: "1.1" };
    pub const HTTP_2_0: super::Protocol<'_> = super::Protocol { protocol: "HTTP", version: "1" };
}

pub fn get_protocol_from_str(protocol_str: &str) -> Result<Protocol<'static>, String> {
    for v in [
        PROTOCOL::HTTP_1_0,
        PROTOCOL::HTTP_1_1,
        PROTOCOL::HTTP_2_0,
    ] {
        if protocol_str == format!("{v}") {
            return Ok(v)
        }
    }
    Err(format!("Unknown status code {protocol_str}."))
}

pub struct Status<'a> {
    pub code: u32,
    pub name: &'a str,
}

impl<'a> std::fmt::Display for Status<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.name.to_uppercase())
    }
}

pub mod STATUS {
    pub const OK: super::Status<'_>        = super::Status { code: 200, name: "OK" };
    pub const NOT_FOUND: super::Status<'_> = super::Status { code: 404, name: "NOT FOUND" };
}

pub fn get_status_from_code(code: u32) -> Result<Status<'static>, String> {
    for v in [
        STATUS::OK,
        STATUS::NOT_FOUND,
    ] {
        if code == v.code {
            return Ok(v)
        }
    }
    Err(format!("Unknown status code {code}."))
}

