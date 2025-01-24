use std::collections::HashMap;
use crate::request::content_type::RawDataType;


#[derive(Clone,Debug)]
pub enum Location {
    Path,
    Body,
    Query,
    Other(String),
    None,
}

impl Location {
    pub fn to_string(&self) -> String {
        match &self {
            Location::Path => String::from("path"),
            Location::Body => String::from("body"),
            Location::Query => String::from("query"),
            Location::Other(v) => v.to_string(),
            _ => String::from("")
        }
    }
    pub fn not(&self, value: &Location) -> bool {
        match &self {
            Location::Path => match value {
                Location::Path => false, _ => true,
            },
            Location::Body => match value {
                Location::Body => false, _ => true,
            },
            Location::Query => match value {
                Location::Query => false, _ => true,
            },
            Location::Other(v) => match value {
                Location::Other(_v) => v != _v, _ => true,
            },
            Location::None => match value {
                Location::None => false, _ => true,
            }
        }
    }
}

#[derive(Debug)]
pub struct ValidationError {
    pub location: Location,
    pub field: Option<String>,
    pub reason: String,
}

pub type ValidationErrors = Vec<ValidationError>;
pub type ValidationResult<T> = Result<T, ValidationErrors>;


pub trait HasDefault {
    fn new() -> Self;
}

#[derive(Clone)]
pub struct Common<T> {
    pub default: Option<T>,
    pub required: bool,
    pub location: Location,
    pub field: Option<String>,
}
impl<T> Common<T> {
    pub fn new() -> Self {
        Self {
            default: None,
            required: true,
            location: Location::None,
            field: None,
        }
    }
}

pub trait FieldValidate {
    type Type: Clone;

    fn common(&self) -> &Common<Self::Type>;
    fn default(&self) -> &Option<Self::Type> { &self.common().default }
    fn required(&self) -> bool { self.common().required }
    fn location(&self) -> Location { self.common().location.clone() }
    fn field(&self) -> Option<String> { self.common().field.clone() }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type>;
    fn validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        self._validate_pre(value)
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
    }
    fn validate_post(&self, value: Self::Type) -> ValidationResult<Self::Type> {
        let mut errs = ValidationErrors::new();
        self._validate_post(&mut errs, &value);
        if errs.len() == 0 {
            Ok(value)
        } else {
            Err(errs)
        }
    }
    // default
    fn parse(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        Ok(self.validate_post(self.validate_pre(value)?)?)
    }

    fn parse_request(&self,
        request: &crate::request::Request,
        path_args: &HashMap<String,String>,
    ) -> ValidationResult<Option<Self::Type>> {
        let location = self.location();
        let name = self.field().unwrap_or("".to_string()).clone();
        // find variable from location
        match location {
            Location::Query | Location::Path => {
                let src = match location {
                    Location::Query => &request.query,
                    _ => &path_args,
                };
                if let Some(s) = src.get(&name) {
                    match self.parse(RawDataType::Text(s)) {
                        Ok(v) => return Ok(Some(v)),
                        Err(_errs) => return Err(_errs),
                    }
                } else if self.required() && self.default().is_none() {
                    return Err(vec![ValidationError {
                        location: location.clone(),
                        field: Some(name.to_string()),
                        reason: format!(
                            "Missing field \"{name}\" in {}",
                            location.to_string(),
                        ),
                    }])
                } else {
                    match self.default() {
                        Some(default) => return Ok(Some(default.clone())),
                        None => return Ok(None),
                    }
                }
            },
            Location::Body => {
                if let Some(body) = &request.body {
                    match self.parse(body.content()) {
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
}
