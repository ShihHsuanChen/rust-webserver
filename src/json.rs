use serde_json;
use jsonschema;
pub use serde_json::json;

pub type JsonValue = serde_json::Value;

type JsonValidateResult<T> = Result<T,String>;

pub fn parse(s: &str) -> JsonValidateResult<JsonValue> {
    match serde_json::from_str(s) {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from("Given string is not json parsable")),
    }
}

pub fn dump(value: &JsonValue) -> JsonValidateResult<String> {
    match serde_json::to_string(value) {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from("Fail to dump json object to string")),
    }

}

pub struct JsonValidator (JsonValue, jsonschema::Validator);
impl JsonValidator {
    pub fn new(schema: JsonValue) -> JsonValidateResult<Self> {
        match jsonschema::validator_for(&schema) {
            Ok(v) => Ok(Self(schema, v)),
            Err(_) => Err(String::from("Not a valid json schema.")),
        }
    }
    pub fn schema(&self) -> &JsonValue {
        &self.0
    }
    pub fn is_valid(&self, value: &JsonValue) -> bool {
        self.1.is_valid(value)
    }
}

pub fn get_validator_from_schema(schema: JsonValue) -> JsonValidateResult<JsonValidator> {
    JsonValidator::new(schema)
}

pub fn get_validator_from_str(schema_str: &str) -> JsonValidateResult<JsonValidator> {
    get_validator_from_schema(parse(schema_str)?)
}

pub fn validate(validator: &JsonValidator, value: &JsonValue) -> bool {
    validator.is_valid(value)
}

pub fn validate_from_schema(schema: &JsonValue, value: &JsonValue) -> JsonValidateResult<bool> {
    match jsonschema::validator_for(&schema) {
        Ok(validator) => Ok(validator.is_valid(&value)),
        Err(_) => Err(String::from("Not a valid json schema")),
    }
}
