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
    Common,
};
use crate::json;
use crate::request::content_type::{RawDataType, FileCursor};


fn some_if<T,F>(some: &Option<T>, f: F) -> bool
where F: Fn(&T) -> bool {
    match some {
        Some(v) => f(&v),
        None => true,
    }
}


#[derive(Clone)]
pub struct Integer<T> {
    pub common: Common<T>,
    pub gt: Option<T>,
    pub ge: Option<T>,
    pub lt: Option<T>,
    pub le: Option<T>,
    pub choice: Option<Vec<T>>,
} 
impl<T> HasDefault for Integer<T> where T: num::Integer {
    fn new() -> Self {
        Self {
            common: Common::<T>::new(),
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

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
            match value.parse::<T>() {
                Ok(v) => Ok(v),
                Err(_) => Err(vec![ValidationError {
                    location: self.location(),
                    field: None,
                    reason: format!(
                        "value cannot be converted to {} type",
                        std::any::type_name::<T>()
                    ),
                }]),
            }
        } else {
            panic!("Raw input type should be RawDataType::Text");
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.gt, |gt| value > gt) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should greater than {}",
                    <Option<T> as Clone>::clone(&self.gt).unwrap()
                ),
            });
        }
        if !some_if(&self.ge, |ge| value >= ge) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be greater equal than {}",
                    <Option<T> as Clone>::clone(&self.ge).unwrap()
                ),
            });
        }
        if !some_if(&self.lt, |lt| value < lt) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be less than {}",
                    <Option<T> as Clone>::clone(&self.lt).unwrap()
                ),
            });
        }
        if !some_if(&self.le, |le| value <= le) {
            errs.push(ValidationError {
                location: self.location().clone(), field: None, reason: format!(
                    "Value should be less equal than {}",
                    <Option<T> as Clone>::clone(&self.le).unwrap()
                ),
            });
        }
        if !some_if(&self.choice, |choice| choice.contains(value)) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be one of {:?}",
                    <Option<Vec<T>> as Clone>::clone(&self.choice).unwrap()
                ),
            });
        }
    }
}


#[derive(Clone)]
pub struct Float<T> {
    pub common: Common<T>,
    pub gt: Option<T>,
    pub ge: Option<T>,
    pub lt: Option<T>,
    pub le: Option<T>,
} 
impl<T> HasDefault for Float<T> where T: num::Float {
    fn new() -> Self {
        Self {
            common: Common::<T>::new(),
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

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
            match value.parse::<T>() {
                Ok(v) => Ok(v),
                Err(_) => Err(vec![ValidationError {
                    location: self.location(),
                    field: None,
                    reason: format!(
                        "value cannot be converted to {} type",
                        std::any::type_name::<T>()
                    ),
                }]),
            }
        } else {
            panic!("Raw input type should be RawDataType::Text");
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.gt, |gt| value > gt) {
            errs.push(ValidationError {
                location: self.location(), field: None, reason: format!(
                    "Value should greater than {}",
                    <Option<T> as Clone>::clone(&self.gt).unwrap()
                ),
            });
        }
        if !some_if(&self.ge, |ge| value >= ge) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be greater equal than {}",
                    <Option<T> as Clone>::clone(&self.ge).unwrap()
                ),
            });
        }
        if !some_if(&self.lt, |lt| value < lt) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be less than {}",
                    <Option<T> as Clone>::clone(&self.lt).unwrap()
                ),
            });
        }
        if !some_if(&self.le, |le| value <= le) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be less equal than {}",
                    <Option<T> as Clone>::clone(&self.le).unwrap()
                ),
            });
        }
    }
}

#[derive(Clone)]
pub struct Text {
    pub common: Common<String>,
    pub min_len: Option<usize>,
    pub max_len: Option<usize>,
    pub pattern: Option<regex::Regex>,
    pub choice: Option<Vec<String>>,
}
impl HasDefault for Text {
     fn new() -> Self {
        Self {
            common: Common::<String>::new(),
            min_len: None,
            max_len: None,
            pattern: None,
            choice: None,
        }
    }
}
impl FieldValidate for Text {
    type Type = String;

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
            Ok(value.to_string())
        } else {
            panic!("Raw input type should be RawDataType::Text");
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should longer equal than {}",
                    <Option<usize> as Clone>::clone(&self.min_len).unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() <= *max_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should shorter equal than {}",
                    <Option<usize> as Clone>::clone(&self.max_len).unwrap()
                ),
            });
        }
        if !some_if(&self.pattern, |pattern| todo!() ) { // TODO: 
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should match pattern {}",
                    <Option<regex::Regex> as Clone>::clone(&self.pattern).unwrap()
                ),
            });
        }
        if !some_if(&self.choice, |choice| choice.contains(value)) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "Value should be one of {:?}",
                    <Option<Vec<String>> as Clone>::clone(&self.choice).unwrap()
                ),
            });
        }
    }
}

#[derive(Clone)]
pub struct Bool {
    pub common: Common<bool>,
}
impl HasDefault for Bool {
    fn new() -> Self {
        Self {
            common: Common::<bool>::new(),
        }
    }
}
impl FieldValidate for Bool {
    type Type = bool;

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
            match &value[..] {
                "true" | "True" | "T" | "1" => Ok(true),
                "false" | "False" | "F" | "0" => Ok(false),
                _ => Err(vec![ValidationError {
                    location: self.location(),
                    field: None,
                    reason: format!("value cannot be converted to bool type"),
                }]),
            }
        } else {
            panic!("Raw input type should be RawDataType::Text");
        }
    }
}


#[derive(Clone)]
pub struct AnyJson {
    pub common: Common<json::JsonValue>,
    pub schema: Option<json::JsonValue>,
}
impl HasDefault for AnyJson {
    fn new() -> Self {
        Self {
            common: Common::<json::JsonValue>::new(),
            schema: None,
        }
    }
}
impl FieldValidate for AnyJson {
    type Type = json::JsonValue;

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
            match json::parse(value) {
                Ok(obj) => Ok(obj),
                Err(_) => Err( vec![ValidationError {
                    location: self.location(),
                    field: None,
                    reason: format!("value is not json-parsable."),
                }])
            }
        } else {
            panic!("Raw input type should be RawDataType::Text");
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if let Some(schema) = &self.schema {
            let validator = match json::JsonValidator::new(schema.clone()) {
                Ok(v) => v,
                Err(e) => {
                    errs.push(ValidationError {
                        location: self.location(),
                        field: None,
                        reason: e.to_string(),
                    });
                    return;
                }
            };
            if !validator.is_valid(value) {
                errs.push(ValidationError {
                    location: self.location(),
                    field: None,
                    reason: format!("value is not match the schema"),
                });
            }
        }
    }
}


#[derive(Clone)]
pub struct File {
    pub common: Common<FileCursor>,
    pub allowed_exts: Option<Vec<String>>,
}
impl HasDefault for File {
    fn new() -> Self {
        Self {
            common: Common::<FileCursor>::new(),
            allowed_exts: None,
        }
    }
}
impl FieldValidate for File {
    type Type = FileCursor;

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::File(value) = value {
            Ok(value.clone()) // TODO: better way that don't clone
        } else {
            panic!("Raw input type should be RawDataType::Binary");
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        let ext = value.ext();
        if !some_if(&self.allowed_exts, |allowed_ext| allowed_ext.contains(&ext)) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "File extension {} is not allowed",
                    ext,
                ),
            });
        }
    }
}

#[derive(Clone)]
pub struct Binary {
    pub common: Common<Vec<u8>>,
}
impl HasDefault for Binary {
    fn new() -> Self {
        Self {
            common: Common::<Vec<u8>>::new(),
        }
    }
}
impl FieldValidate for Binary {
    type Type = Vec<u8>;

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Binary(value) = value {
            Ok(value.to_vec())
        } else {
            panic!("Raw input type should be RawDataType::Binary");
        }
    }
}


#[derive(Clone)]
pub struct Array<T>
where T: FieldValidate
{
    pub common: Common<Vec<T::Type>>,
    pub elem_field: T,
    pub min_len: Option<usize>,
    pub max_len: Option<usize>,
}
impl<T> Array<T>
where T: FieldValidate + HasDefault
{
    pub fn new() -> Self {
        Self {
            common: Common::<Vec<T::Type>>::new(),
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

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        if let RawDataType::Text(value) = value {
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
                            match self.elem_field.parse(RawDataType::Text(&s)) {
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
                                location: self.location(),
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
                    location: self.location(),
                    field: None,
                    reason: format!("value is not a valid array"),
                }]),
            }
        } else {
            panic!();
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "array length should greater equal than {}",
                    self.min_len.unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() >= *max_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "array length should greater equal than {}",
                    self.max_len.unwrap()
                ),
            });
        }
    }
}


#[derive(Clone)]
pub struct Mapping<T>
where T: FieldValidate
{
    pub common: Common<HashMap<String,T::Type>>,
    pub elem_field: T,
    pub min_len: Option<usize>,
    pub max_len: Option<usize>,
}
impl<T> Mapping<T>
where T: FieldValidate + HasDefault
{
    pub fn new() -> Self {
        Self {
            common: Common::<HashMap<String,T::Type>>::new(),
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

    fn common(&self) -> &Common<Self::Type> { &self.common }

    fn _validate_pre(&self, value: RawDataType) -> ValidationResult<Self::Type> {
        match value {
            RawDataType::Text(value) => {
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
                                match self.elem_field.parse(RawDataType::Text(&s)) {
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
                                    location: self.location(),
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
                        location: self.location(),
                        field: None,
                        reason: format!("value is not a valid array"),
                    }]),
                }
            },
            _ => panic!("Raw input type should be RawDataType::Text"),
            
        }
    }
    fn _validate_post(&self, errs: &mut ValidationErrors, value: &Self::Type) {
        if !some_if(&self.min_len, |min_len| value.len() >= *min_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "array length should greater equal than {}",
                    self.min_len.unwrap()
                ),
            });
        }
        if !some_if(&self.max_len, |max_len| value.len() >= *max_len) {
            errs.push(ValidationError {
                location: self.location(),
                field: None,
                reason: format!(
                    "array length should greater equal than {}",
                    self.max_len.unwrap()
                ),
            });
        }
    }
}
