

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
}

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

pub trait FieldValidate {
    type Type;

    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type>;
    fn validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
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
    fn parse(&self, value: &str) -> ValidationResult<Self::Type> {
        Ok(self.validate_post(self.validate_pre(value)?)?)
    }
}
