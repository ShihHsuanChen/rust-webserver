use std::collections::HashMap;

pub type Headers = HashMap<String,(String,Vec<String>)>;

pub const CRLF: &str = "\r\n";
pub const STATUS_SP: &str = " ";
pub const HEADER_SP: &str = ":";
pub const HEADER_META_SP: &str = ";";


pub fn split_header_line(line: &str) -> Result<((&str, &str), Vec<&str>), String> {
    // format:
    //   header-name: header-value*(; metadata)
    // example:
    //   Content-Disposition: form-data; name=\"b\"
    //   Content-Disposition: form-data; name=\"b\"; filename=\"abcabcabc.docx\"

    let segs: Vec<_> = line.split(HEADER_META_SP).map(|v|{v.trim()}).collect();
    if let Some(kv) = segs[0].split_once(HEADER_SP) {
        let (k, v) = kv;
        let (k, v) = (k.trim(), v.trim());
        let mut metadata = Vec::<&str>::new();
        if segs.len() > 1 {
            for seg in &segs[1..] {
                metadata.push(seg);
            }
        }
        Ok(((k,v), metadata))
    } else {
        Err(String::from("Fail to split header line"))
    }
}


pub fn is_CRLF_str(s: &str) -> bool {
    s == CRLF
}

pub fn is_CRLF_bytes(s: &[u8]) -> bool {
    s == CRLF.as_bytes()
}
