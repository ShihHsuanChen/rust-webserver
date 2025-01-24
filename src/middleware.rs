use std::collections::HashMap;

use crate::request::{
    Request,
    content_type::RawDataType,
};
use crate::schema::{
    Location,
    ValidationResult,
    ValidationError,
    FieldValidate,
};


pub fn parse_request<T>(
    field: T,
    request: &Request,
    path_args: &HashMap<String,String>,
) -> ValidationResult<Option<T::Type>>
where T: FieldValidate
{
    let location = field.location();
    let name = field.field().unwrap_or("".to_string()).clone();
    // find variable from location
    match location {
        Location::Query | Location::Path => {
            let src = match location {
                Location::Query => &request.query,
                _ => &path_args,
            };
            if let Some(s) = src.get(&name) {
                match field.parse(RawDataType::Text(s)) {
                    Ok(v) => return Ok(Some(v)),
                    Err(_errs) => return Err(_errs),
                }
            } else if field.required() && field.default().is_none() {
                return Err(vec![ValidationError {
                    location: location.clone(),
                    field: Some(name.to_string()),
                    reason: format!(
                        "Missing field \"{name}\" in {}",
                        location.to_string(),
                    ),
                }])
            } else {
                match field.default() {
                    Some(default) => return Ok(Some(default.clone())),
                    None => return Ok(None),
                }
            }
        },
        Location::Body => {
            if let Some(body) = &request.body {
                match field.parse(body.content()) {
                    Ok(v) => return Ok(Some(v)),
                    Err(_errs) => return Err(_errs),
                }
            } else {
                return Err(vec![ValidationError {
                    location: Location::None,
                    field: Some(name.to_string()),
                    reason: format!(
                        "Request body is empty",
                    ),
                }]);
            }
        },
        _ => {
            return Err(vec![ValidationError {
                location: Location::None,
                field: Some(name.to_string()),
                reason: format!(
                    "Invalid location {}",
                    location.to_string()
                ),
            }]);
        },
    };
}

