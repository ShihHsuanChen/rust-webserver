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
}
