use std::collections::HashMap;

use num;
use regex;

use super::base::{
    Location,
    ValidationError,
    ValidationErrors,
    ValidationResult,
    FieldValidate,
    HasDefault,
};
use crate::json;


fn some_if<T,F>(some: &Option<T>, f: F) -> bool
where F: Fn(&T) -> bool {
    match some {
        Some(v) => f(&v),
        None => true,
    }
}


pub struct Integer<T> {
    default: Option<T>,
    required: bool,
    gt: Option<T>,
    ge: Option<T>,
    lt: Option<T>,
    le: Option<T>,
    choice: Option<Vec<T>>,
} 
impl<T> HasDefault for Integer<T> where T: num::Integer {
    fn new() -> Self {
        Self {
            default: None,
            required: true,
            gt: None,
            ge: None,
            lt: None,
            le: None,
            choice: None,
        }
    }
}
impl<T> FieldValidate for Integer<T>
where T: num::Integer + std::str::FromStr + std::fmt::Display + std::fmt::Debug + Clone {
    type Type = T;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match value.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(vec![ValidationError {
                location: Location::None, field: None, reason: format!(
                    "value cannot be converted to {} type",
                    std::any::type_name::<T>()
                ),
            }]),
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.gt, |gt| value > gt) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should greater than {}",
                    <Option<T> as Clone>::clone(&self.gt).unwrap()
                ),
            });
        }
        if !some_if(&self.ge, |ge| value >= ge) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be greater equal than {}",
                    <Option<T> as Clone>::clone(&self.ge).unwrap()
                ),
            });
        }
        if !some_if(&self.lt, |lt| value < lt) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be less than {}",
                    <Option<T> as Clone>::clone(&self.lt).unwrap()
                ),
            });
        }
        if !some_if(&self.le, |le| value <= le) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be less equal than {}",
                    <Option<T> as Clone>::clone(&self.le).unwrap()
                ),
            });
        }
        if !some_if(&self.choice, |choice| choice.contains(value)) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be one of {:?}",
                    <Option<Vec<T>> as Clone>::clone(&self.choice).unwrap()
                ),
            });
        }
    }
}


pub struct Float<T> {
    default: Option<T>,
    required: bool,
    gt: Option<T>,
    ge: Option<T>,
    lt: Option<T>,
    le: Option<T>,
} 
impl<T> HasDefault for Float<T> where T: num::Float {
    fn new() -> Self {
        Self {
            default: None,
            required: true,
            gt: None,
            ge: None,
            lt: None,
            le: None,
        }
    }
}
impl<T> FieldValidate for Float<T>
where T: num::Float + std::str::FromStr + std::fmt::Display + std::fmt::Debug + Clone {
    type Type = T;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match value.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(vec![ValidationError {
                location: Location::None,
                field: None,
                reason: format!("value cannot be converted to {} type", std::any::type_name::<T>()),
            }]),
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.gt, |gt| value > gt) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should greater than {}",
                    <Option<T> as Clone>::clone(&self.gt).unwrap()
                ),
            });
        }
        if !some_if(&self.ge, |ge| value >= ge) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be greater equal than {}",
                    <Option<T> as Clone>::clone(&self.ge).unwrap()
                ),
            });
        }
        if !some_if(&self.lt, |lt| value < lt) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be less than {}",
                    <Option<T> as Clone>::clone(&self.lt).unwrap()
                ),
            });
        }
        if !some_if(&self.le, |le| value <= le) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be less equal than {}",
                    <Option<T> as Clone>::clone(&self.le).unwrap()
                ),
            });
        }
    }
}

pub struct Text {
    default: Option<String>,
    required: bool,
    min_len: Option<usize>,
    max_len: Option<usize>,
    pattern: Option<regex::Regex>,
    choice: Option<Vec<String>>,
}
impl HasDefault for Text {
     fn new() -> Self {
        Self {
            default: None,
            required: true,
            min_len: None,
            max_len: None,
            pattern: None,
            choice: None,
        }
    }
}
impl FieldValidate for Text {
    type Type = String;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        Ok(value.to_string())
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should longer equal than {}",
                    <Option<usize> as Clone>::clone(&self.min_len).unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() <= *max_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should shorter equal than {}",
                    <Option<usize> as Clone>::clone(&self.max_len).unwrap()
                ),
            });
        }
        if !some_if(&self.pattern, |pattern| todo!() ) { // TODO: 
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should match pattern {}",
                    <Option<regex::Regex> as Clone>::clone(&self.pattern).unwrap()
                ),
            });
        }
        if !some_if(&self.choice, |choice| choice.contains(value)) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "Value should be one of {:?}",
                    <Option<Vec<String>> as Clone>::clone(&self.choice).unwrap()
                ),
            });
        }
    }
}

pub struct Bool {
    default: Option<bool>,
    required: bool,
}
impl HasDefault for Bool {
    fn new() -> Self {
        Self {
            default: None,
            required: true,
        }
    }
}
impl FieldValidate for Bool {
    type Type = bool;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match value {
            "true" | "True" | "T" | "1" => Ok(true),
            "false" | "False" | "F" | "0" => Ok(false),
            _ => Err(vec![ValidationError {
                location: Location::None,
                field: None,
                reason: format!("value cannot be converted to bool type"),
            }]),
        }
    }
}


pub struct AnyJson {
    default: Option<json::JsonValue>,
    required: bool,
    schema: Option<json::JsonValidator>,
}
impl HasDefault for AnyJson {
    fn new() -> Self {
        Self {
            default: None,
            required: true,
            schema: None,
        }
    }
}
impl FieldValidate for AnyJson {
    type Type = json::JsonValue;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match json::parse(value) {
            Ok(obj) => Ok(obj),
            Err(_) => Err( vec![ValidationError {
                location: Location::None,
                field: None,
                reason: format!("value is not json-parsable."),
            }])
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.schema, |validator| validator.is_valid(value)) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "value is not match the schema",
                ),
            });
        }
    }
}


// pub struct File {
//     default: Option<Vec<u8>>,
//     required: bool,
//     allowed_exts: Option<Vec<String>>,
// }
// impl FieldValidate for File {
// }

// pub struct Binary {
//     default: Option<Vec<u8>>,
//     required: bool,
// }
// impl FieldValidate for Binary {
// }


pub struct Array<T>
where T: FieldValidate
{
    default: Option<Vec<T::Type>>,
    required: bool,
    elem_field: T,
    min_len: Option<usize>,
    max_len: Option<usize>,
}
impl<T> Array<T>
where T: FieldValidate + HasDefault
{
    pub fn new() -> Self {
        Self {
            default: None,
            required: true,
            elem_field: T::new(),
            min_len: None,
            max_len: None,
        }
    }
}
impl<T> FieldValidate for Array<T>
where T: FieldValidate
{
    type Type = Vec<T::Type>;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match json::parse(value) {
            Ok(json::JsonValue::Array(arr)) => {
                let mut res = Self::Type::new();
                let mut errs = ValidationErrors::new();
                for (i, el) in arr.iter().enumerate() {
                    if let Ok(mut s) = json::dump(el) {
                        // keep outer \"\" only if the elem_field::Type is JsonValue
                        if std::any::type_name::<T::Type>() != std::any::type_name::<json::JsonValue>() {
                            s = s.trim_matches('\"').to_string();
                        }
                        //
                        match self.elem_field.parse(&s) {
                            Ok(v) => res.push(v),
                            Err(_errs) => {
                                for mut _err in _errs {
                                    let field = Some(match _err.field {
                                        Some(_field) => format!("{i}.{_field}"),
                                        None => i.to_string(),
                                    });
                                    _err.field = field;
                                    errs.push(_err);
                                }
                            }
                        }
                    } else {
                        errs.push(ValidationError {
                            location: Location::None,
                            field: Some(i.to_string()),
                            reason: format!(
                                "value cannot be converted to {} type",
                                std::any::type_name::<T>()
                            ),
                        });
                    }
                }
                if errs.len() == 0 {
                    Ok(res)
                } else {
                    Err(errs)
                }
            },
            _ => Err(vec![ValidationError {
                location: Location::None,
                field: None,
                reason: format!("value is not a valid array"),
            }]),
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "array length should greater equal than {}",
                    self.min_len.unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() >= *max_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "array length should greater equal than {}",
                    self.max_len.unwrap()
                ),
            });
        }
    }
}


pub struct Mapping<T>
where T: FieldValidate
{
    default: Option<HashMap<String,T::Type>>,
    required: bool,
    elem_field: T,
    min_len: Option<usize>,
    max_len: Option<usize>,
}
impl<T> Mapping<T>
where T: FieldValidate + HasDefault
{
    pub fn new() -> Self {
        Self {
            default: None,
            required: true,
            elem_field: T::new(),
            min_len: None,
            max_len: None,
        }
    }
}
impl<T> FieldValidate for Mapping<T>
where T: FieldValidate
{
    type Type = HashMap<String,T::Type>;
    fn _validate_pre(&self, value: &str) -> ValidationResult<Self::Type> {
        match json::parse(value) {
            Ok(json::JsonValue::Object(obj)) => {
                let mut res = Self::Type::new();
                let mut errs = ValidationErrors::new();
                for (k, el) in obj.iter() {
                    let k = k.clone();
                    if let Ok(mut s) = json::dump(el) {
                        // keep outer \"\" only if the elem_field::Type is JsonValue
                        if std::any::type_name::<T::Type>() != std::any::type_name::<json::JsonValue>() {
                            s = s.trim_matches('\"').to_string();
                        }
                        //
                        match self.elem_field.parse(&s) {
                            Ok(v) => { res.insert(k, v); },
                            Err(_errs) => {
                                for mut _err in _errs {
                                    let _k = k.clone();
                                    let field = Some(match _err.field {
                                        Some(_field) => format!("{}.{_field}",_k.clone()),
                                        None => _k,
                                    });
                                    _err.field = field;
                                    errs.push(_err);
                                }
                            }
                        }
                    } else {
                        errs.push(ValidationError {
                            location: Location::None,
                            field: Some(k.to_string()),
                            reason: format!(
                                "value cannot be converted to {} type",
                                std::any::type_name::<T>()
                            ),
                        });
                    }
                }
                if errs.len() == 0 {
                    Ok(res)
                } else {
                    Err(errs)
                }
            },
            _ => Err(vec![ValidationError {
                location: Location::None,
                field: None,
                reason: format!("value is not a valid array"),
            }]),
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "array length should greater equal than {}",
                    self.min_len.unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() >= *max_len) {
            errs.push(ValidationError {
                location: Location::None, field: None, reason: format!(
                    "array length should greater equal than {}",
                    self.max_len.unwrap()
                ),
            });
        }
    }
}
