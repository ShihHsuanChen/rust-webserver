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
    pub const GET: super::Method<'_>     = super::Method("GET");
    pub const POST: super::Method<'_>    = super::Method("POST");
    pub const PUT: super::Method<'_>     = super::Method("PUT");
    pub const PATCH: super::Method<'_>   = super::Method("PATCH");
    pub const DELETE: super::Method<'_>  = super::Method("DELETE");
    pub const HEAD: super::Method<'_>    = super::Method("HEAD");
    pub const OPTIONS: super::Method<'_> = super::Method("OPTIONS");
    pub const CONNECT: super::Method<'_> = super::Method("CONNECT");
    pub const TRACE: super::Method<'_>   = super::Method("TRACE");
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
    use super::Status;
    pub const CONTINUE:                        Status<'_> = Status { code: 100, name: "CONTINUE" };
    pub const SWITCHING_PROTOCOLS:             Status<'_> = Status { code: 101, name: "SWITCHING PROTOCOLS" };
    pub const PROCESSING:                      Status<'_> = Status { code: 102, name: "PROCESSING" };
    pub const EARLY_HINTS:                     Status<'_> = Status { code: 103, name: "EARLY HINTS" };
    pub const OK:                              Status<'_> = Status { code: 200, name: "OK" };
    pub const CREATED:                         Status<'_> = Status { code: 201, name: "CREATED" };
    pub const ACCEPTED:                        Status<'_> = Status { code: 202, name: "ACCEPTED" };
    pub const NON_AUTHORITATIVE_INFORMATION:   Status<'_> = Status { code: 203, name: "NON-AUTHORITATIVE INFORMATION" };
    pub const NO_CONTENT:                      Status<'_> = Status { code: 204, name: "NO CONTENT" };
    pub const RESET_CONTENT:                   Status<'_> = Status { code: 205, name: "RESET CONTENT" };
    pub const PARTIAL_CONTENT:                 Status<'_> = Status { code: 206, name: "PARTIAL CONTENT" };
    pub const MULTI_STATUS:                    Status<'_> = Status { code: 207, name: "MULTI-STATUS" };
    pub const ALREADY_REPORTED:                Status<'_> = Status { code: 208, name: "ALREADY REPORTED" };
    pub const IM_USED:                         Status<'_> = Status { code: 226, name: "IM USED" };
    pub const MULTIPLE_CHOICES:                Status<'_> = Status { code: 300, name: "MULTIPLE CHOICES" };
    pub const MOVED_PERMANENTLY:               Status<'_> = Status { code: 301, name: "MOVED PERMANENTLY" };
    pub const FOUND:                           Status<'_> = Status { code: 302, name: "FOUND" };
    pub const SEE_OTHER:                       Status<'_> = Status { code: 303, name: "SEE OTHER" };
    pub const NOT_MODIFIED:                    Status<'_> = Status { code: 304, name: "NOT MODIFIED" };
    pub const USE_PROXY:                       Status<'_> = Status { code: 305, name: "USE PROXY" };
    pub const UNUSED:                          Status<'_> = Status { code: 306, name: "UNUSED" };
    pub const TEMPORARY_REDIRECT:              Status<'_> = Status { code: 307, name: "TEMPORARY REDIRECT" };
    pub const PERMANENT_REDIRECT:              Status<'_> = Status { code: 308, name: "PERMANENT REDIRECT" };
    pub const BAD_REQUEST:                     Status<'_> = Status { code: 400, name: "BAD REQUEST" };
    pub const UNAUTHORIZED:                    Status<'_> = Status { code: 401, name: "UNAUTHORIZED" };
    pub const PAYMENT_REQUIRED:                Status<'_> = Status { code: 402, name: "PAYMENT REQUIRED" };
    pub const FORBIDDEN:                       Status<'_> = Status { code: 403, name: "FORBIDDEN" };
    pub const NOT_FOUND:                       Status<'_> = Status { code: 404, name: "NOT FOUND" };
    pub const METHOD_NOT_ALLOWED:              Status<'_> = Status { code: 405, name: "METHOD NOT ALLOWED" };
    pub const NOT_ACCEPTABLE:                  Status<'_> = Status { code: 406, name: "NOT ACCEPTABLE" };
    pub const PROXY_AUTHENTICATION_REQUIRED:   Status<'_> = Status { code: 407, name: "PROXY AUTHENTICATION REQUIRED" };
    pub const REQUEST_TIMEOUT:                 Status<'_> = Status { code: 408, name: "REQUEST TIMEOUT" };
    pub const CONFLICT:                        Status<'_> = Status { code: 409, name: "CONFLICT" };
    pub const GONE:                            Status<'_> = Status { code: 410, name: "GONE" };
    pub const LENGTH_REQUIRED:                 Status<'_> = Status { code: 411, name: "LENGTH REQUIRED" };
    pub const PRECONDITION_FAILED:             Status<'_> = Status { code: 412, name: "PRECONDITION FAILED" };
    pub const CONTENT_TOO_LARGE:               Status<'_> = Status { code: 413, name: "CONTENT TOO LARGE" };
    pub const URI_TOO_LONG:                    Status<'_> = Status { code: 414, name: "URI TOO LONG" };
    pub const UNSUPPORTED_MEDIA_TYPE:          Status<'_> = Status { code: 415, name: "UNSUPPORTED MEDIA TYPE" };
    pub const RANGE_NOT_SATISFIABLE:           Status<'_> = Status { code: 416, name: "RANGE NOT SATISFIABLE" };
    pub const EXPECTATION_FAILED:              Status<'_> = Status { code: 417, name: "EXPECTATION FAILED" };
    pub const IM_A_TEAPOT:                     Status<'_> = Status { code: 418, name: "IM A TEAPOT" };
    pub const MISDIRECTED_REQUEST:             Status<'_> = Status { code: 421, name: "MISDIRECTED REQUEST" };
    pub const UNPROCESSABLE_CONTENT:           Status<'_> = Status { code: 422, name: "UNPROCESSABLE CONTENT" };
    pub const LOCKED:                          Status<'_> = Status { code: 423, name: "LOCKED" };
    pub const FAILED_DEPENDENCY:               Status<'_> = Status { code: 424, name: "FAILED DEPENDENCY" };
    pub const TOO_EARLY:                       Status<'_> = Status { code: 425, name: "TOO EARLY" };
    pub const UPGRADE_REQUIRED:                Status<'_> = Status { code: 426, name: "UPGRADE REQUIRED" };
    pub const PRECONDITION_REQUIRED:           Status<'_> = Status { code: 428, name: "PRECONDITION REQUIRED" };
    pub const TOO_MANY_REQUESTS:               Status<'_> = Status { code: 429, name: "TOO MANY REQUESTS" };
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Status<'_> = Status { code: 431, name: "REQUEST HEADER FIELDS TOO LARGE" };
    pub const UNAVAILABLE_FOR_LEGAL_REASONS:   Status<'_> = Status { code: 451, name: "UNAVAILABLE FOR LEGAL REASONS" };
    pub const INTERNAL_SERVER_ERROR:           Status<'_> = Status { code: 500, name: "INTERNAL SERVER ERROR" };
    pub const NOT_IMPLEMENTED:                 Status<'_> = Status { code: 501, name: "NOT IMPLEMENTED" };
    pub const BAD_GATEWAY:                     Status<'_> = Status { code: 502, name: "BAD GATEWAY" };
    pub const SERVICE_UNAVAILABLE:             Status<'_> = Status { code: 503, name: "SERVICE UNAVAILABLE" };
    pub const GATEWAY_TIMEOUT:                 Status<'_> = Status { code: 504, name: "GATEWAY TIMEOUT" };
    pub const HTTP_VERSION_NOT_SUPPORTED:      Status<'_> = Status { code: 505, name: "HTTP VERSION NOT SUPPORTED" };
    pub const VARIANT_ALSO_NEGOTIATES:         Status<'_> = Status { code: 506, name: "VARIANT ALSO NEGOTIATES" };
    pub const INSUFFICIENT_STORAGE:            Status<'_> = Status { code: 507, name: "INSUFFICIENT STORAGE" };
    pub const LOOP_DETECTED:                   Status<'_> = Status { code: 508, name: "LOOP DETECTED" };
    pub const NOT_EXTENDED:                    Status<'_> = Status { code: 510, name: "NOT EXTENDED" };
    pub const NETWORK_AUTHENTICATION_REQUIRED: Status<'_> = Status { code: 511, name: "NETWORK AUTHENTICATION REQUIRED" };
}

pub fn get_status_from_code(code: u32) -> Result<Status<'static>, String> {
    for v in [
        STATUS::CONTINUE,
        STATUS::SWITCHING_PROTOCOLS,
        STATUS::PROCESSING,
        STATUS::EARLY_HINTS,
        STATUS::OK,
        STATUS::CREATED,
        STATUS::ACCEPTED,
        STATUS::NON_AUTHORITATIVE_INFORMATION,
        STATUS::NO_CONTENT,
        STATUS::RESET_CONTENT,
        STATUS::PARTIAL_CONTENT,
        STATUS::MULTI_STATUS,
        STATUS::ALREADY_REPORTED,
        STATUS::IM_USED,
        STATUS::MULTIPLE_CHOICES,
        STATUS::MOVED_PERMANENTLY,
        STATUS::FOUND,
        STATUS::SEE_OTHER,
        STATUS::NOT_MODIFIED,
        STATUS::USE_PROXY,
        STATUS::UNUSED,
        STATUS::TEMPORARY_REDIRECT,
        STATUS::PERMANENT_REDIRECT,
        STATUS::BAD_REQUEST,
        STATUS::UNAUTHORIZED,
        STATUS::PAYMENT_REQUIRED,
        STATUS::FORBIDDEN,
        STATUS::NOT_FOUND,
        STATUS::METHOD_NOT_ALLOWED,
        STATUS::NOT_ACCEPTABLE,
        STATUS::PROXY_AUTHENTICATION_REQUIRED,
        STATUS::REQUEST_TIMEOUT,
        STATUS::CONFLICT,
        STATUS::GONE,
        STATUS::LENGTH_REQUIRED,
        STATUS::PRECONDITION_FAILED,
        STATUS::CONTENT_TOO_LARGE,
        STATUS::URI_TOO_LONG,
        STATUS::UNSUPPORTED_MEDIA_TYPE,
        STATUS::RANGE_NOT_SATISFIABLE,
        STATUS::EXPECTATION_FAILED,
        STATUS::IM_A_TEAPOT,
        STATUS::MISDIRECTED_REQUEST,
        STATUS::UNPROCESSABLE_CONTENT,
        STATUS::LOCKED,
        STATUS::FAILED_DEPENDENCY,
        STATUS::TOO_EARLY,
        STATUS::UPGRADE_REQUIRED,
        STATUS::PRECONDITION_REQUIRED,
        STATUS::TOO_MANY_REQUESTS,
        STATUS::REQUEST_HEADER_FIELDS_TOO_LARGE,
        STATUS::UNAVAILABLE_FOR_LEGAL_REASONS,
        STATUS::INTERNAL_SERVER_ERROR,
        STATUS::NOT_IMPLEMENTED,
        STATUS::BAD_GATEWAY,
        STATUS::SERVICE_UNAVAILABLE,
        STATUS::GATEWAY_TIMEOUT,
        STATUS::HTTP_VERSION_NOT_SUPPORTED,
        STATUS::VARIANT_ALSO_NEGOTIATES,
        STATUS::INSUFFICIENT_STORAGE,
        STATUS::LOOP_DETECTED,
        STATUS::NOT_EXTENDED,
        STATUS::NETWORK_AUTHENTICATION_REQUIRED,
    ] {
        if code == v.code {
            return Ok(v)
        }
    }
    Err(format!("Unknown status code {code}."))
}

