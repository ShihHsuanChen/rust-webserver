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
    use super::Status;
    pub const CONTINUE:                        Status<'_> = Status { code: 100, name: "CONTINUE" };
    pub const SWITCHING_PROTOCOLS:             Status<'_> = Status { code: 101, name: "SWITCHING_PROTOCOLS" };
    pub const PROCESSING:                      Status<'_> = Status { code: 102, name: "PROCESSING" };
    pub const EARLY_HINTS:                     Status<'_> = Status { code: 103, name: "EARLY_HINTS" };
    pub const OK:                              Status<'_> = Status { code: 200, name: "OK" };
    pub const CREATED:                         Status<'_> = Status { code: 201, name: "CREATED" };
    pub const ACCEPTED:                        Status<'_> = Status { code: 202, name: "ACCEPTED" };
    pub const NON_AUTHORITATIVE_INFORMATION:   Status<'_> = Status { code: 203, name: "NON-AUTHORITATIVE_INFORMATION" };
    pub const NO_CONTENT:                      Status<'_> = Status { code: 204, name: "NO_CONTENT" };
    pub const RESET_CONTENT:                   Status<'_> = Status { code: 205, name: "RESET_CONTENT" };
    pub const PARTIAL_CONTENT:                 Status<'_> = Status { code: 206, name: "PARTIAL_CONTENT" };
    pub const MULTI_STATUS:                    Status<'_> = Status { code: 207, name: "MULTI-STATUS" };
    pub const ALREADY_REPORTED:                Status<'_> = Status { code: 208, name: "ALREADY_REPORTED" };
    pub const IM_USED:                         Status<'_> = Status { code: 226, name: "IM_USED" };
    pub const MULTIPLE_CHOICES:                Status<'_> = Status { code: 300, name: "MULTIPLE_CHOICES" };
    pub const MOVED_PERMANENTLY:               Status<'_> = Status { code: 301, name: "MOVED_PERMANENTLY" };
    pub const FOUND:                           Status<'_> = Status { code: 302, name: "FOUND" };
    pub const SEE_OTHER:                       Status<'_> = Status { code: 303, name: "SEE_OTHER" };
    pub const NOT_MODIFIED:                    Status<'_> = Status { code: 304, name: "NOT_MODIFIED" };
    pub const USE_PROXY:                       Status<'_> = Status { code: 305, name: "USE_PROXY" };
    pub const UNUSED:                          Status<'_> = Status { code: 306, name: "UNUSED" };
    pub const TEMPORARY_REDIRECT:              Status<'_> = Status { code: 307, name: "TEMPORARY_REDIRECT" };
    pub const PERMANENT_REDIRECT:              Status<'_> = Status { code: 308, name: "PERMANENT_REDIRECT" };
    pub const BAD_REQUEST:                     Status<'_> = Status { code: 400, name: "BAD_REQUEST" };
    pub const UNAUTHORIZED:                    Status<'_> = Status { code: 401, name: "UNAUTHORIZED" };
    pub const PAYMENT_REQUIRED:                Status<'_> = Status { code: 402, name: "PAYMENT_REQUIRED" };
    pub const FORBIDDEN:                       Status<'_> = Status { code: 403, name: "FORBIDDEN" };
    pub const NOT_FOUND:                       Status<'_> = Status { code: 404, name: "NOT_FOUND" };
    pub const METHOD_NOT_ALLOWED:              Status<'_> = Status { code: 405, name: "METHOD_NOT_ALLOWED" };
    pub const NOT_ACCEPTABLE:                  Status<'_> = Status { code: 406, name: "NOT_ACCEPTABLE" };
    pub const PROXY_AUTHENTICATION_REQUIRED:   Status<'_> = Status { code: 407, name: "PROXY_AUTHENTICATION_REQUIRED" };
    pub const REQUEST_TIMEOUT:                 Status<'_> = Status { code: 408, name: "REQUEST_TIMEOUT" };
    pub const CONFLICT:                        Status<'_> = Status { code: 409, name: "CONFLICT" };
    pub const GONE:                            Status<'_> = Status { code: 410, name: "GONE" };
    pub const LENGTH_REQUIRED:                 Status<'_> = Status { code: 411, name: "LENGTH_REQUIRED" };
    pub const PRECONDITION_FAILED:             Status<'_> = Status { code: 412, name: "PRECONDITION_FAILED" };
    pub const CONTENT_TOO_LARGE:               Status<'_> = Status { code: 413, name: "CONTENT_TOO_LARGE" };
    pub const URI_TOO_LONG:                    Status<'_> = Status { code: 414, name: "URI_TOO_LONG" };
    pub const UNSUPPORTED_MEDIA_TYPE:          Status<'_> = Status { code: 415, name: "UNSUPPORTED_MEDIA_TYPE" };
    pub const RANGE_NOT_SATISFIABLE:           Status<'_> = Status { code: 416, name: "RANGE_NOT_SATISFIABLE" };
    pub const EXPECTATION_FAILED:              Status<'_> = Status { code: 417, name: "EXPECTATION_FAILED" };
    pub const IM_A_TEAPOT:                     Status<'_> = Status { code: 418, name: "IM_A_TEAPOT" };
    pub const MISDIRECTED_REQUEST:             Status<'_> = Status { code: 421, name: "MISDIRECTED_REQUEST" };
    pub const UNPROCESSABLE_CONTENT:           Status<'_> = Status { code: 422, name: "UNPROCESSABLE_CONTENT" };
    pub const LOCKED:                          Status<'_> = Status { code: 423, name: "LOCKED" };
    pub const FAILED_DEPENDENCY:               Status<'_> = Status { code: 424, name: "FAILED_DEPENDENCY" };
    pub const TOO_EARLY:                       Status<'_> = Status { code: 425, name: "TOO_EARLY" };
    pub const UPGRADE_REQUIRED:                Status<'_> = Status { code: 426, name: "UPGRADE_REQUIRED" };
    pub const PRECONDITION_REQUIRED:           Status<'_> = Status { code: 428, name: "PRECONDITION_REQUIRED" };
    pub const TOO_MANY_REQUESTS:               Status<'_> = Status { code: 429, name: "TOO_MANY_REQUESTS" };
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Status<'_> = Status { code: 431, name: "REQUEST_HEADER_FIELDS_TOO_LARGE" };
    pub const UNAVAILABLE_FOR_LEGAL_REASONS:   Status<'_> = Status { code: 451, name: "UNAVAILABLE_FOR_LEGAL_REASONS" };
    pub const INTERNAL_SERVER_ERROR:           Status<'_> = Status { code: 500, name: "INTERNAL_SERVER_ERROR" };
    pub const NOT_IMPLEMENTED:                 Status<'_> = Status { code: 501, name: "NOT_IMPLEMENTED" };
    pub const BAD_GATEWAY:                     Status<'_> = Status { code: 502, name: "BAD_GATEWAY" };
    pub const SERVICE_UNAVAILABLE:             Status<'_> = Status { code: 503, name: "SERVICE_UNAVAILABLE" };
    pub const GATEWAY_TIMEOUT:                 Status<'_> = Status { code: 504, name: "GATEWAY_TIMEOUT" };
    pub const HTTP_VERSION_NOT_SUPPORTED:      Status<'_> = Status { code: 505, name: "HTTP_VERSION_NOT_SUPPORTED" };
    pub const VARIANT_ALSO_NEGOTIATES:         Status<'_> = Status { code: 506, name: "VARIANT_ALSO_NEGOTIATES" };
    pub const INSUFFICIENT_STORAGE:            Status<'_> = Status { code: 507, name: "INSUFFICIENT_STORAGE" };
    pub const LOOP_DETECTED:                   Status<'_> = Status { code: 508, name: "LOOP_DETECTED" };
    pub const NOT_EXTENDED:                    Status<'_> = Status { code: 510, name: "NOT_EXTENDED" };
    pub const NETWORK_AUTHENTICATION_REQUIRED: Status<'_> = Status { code: 511, name: "NETWORK_AUTHENTICATION_REQUIRED" };
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

