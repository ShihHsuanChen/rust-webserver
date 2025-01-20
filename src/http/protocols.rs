use std;


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
