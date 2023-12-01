use std::rc::Rc;
use std::sync::Arc;
use serde_email::Email;
use value_generator::ValueGenerator;
use super::{CellValue, ValueType};
use super::ValueBuilder;

#[derive(Clone, Debug, Default, PartialEq, ValueGenerator)]
pub struct EmailValue {
    value: Email,
}

impl CellValue for EmailValue {
    fn get_value(&self) -> ValueType {
        ValueType::Email(self.clone())
    }
}

#[derive(Default)]
pub struct EmailValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for EmailValueBuilder {
    type Value = EmailValue;
    type RowValueType = Email;
    fn validate(&self) -> Result<Self::RowValueType, String> {
        match self.row_value.as_ref() {
            None => return Err("the value wasn't set up".to_string()),
            Some(_) => ()
        };
        match Email::from_str(self.row_value.as_ref().unwrap().as_str()) {
            Ok(email) => Ok(email),
            Err(err) => Err(format!("couldn't get email: {}", err))
        }
    }

    fn build(self) -> Result<Self::Value, String> {
        match self.validate() {
            Ok(value) => Ok(EmailValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

impl EmailValue {
    pub fn new(value: Email) -> Self {
        Self { value }
    }
    pub fn builder() -> EmailValueBuilder {
        EmailValueBuilder::default()
    }
    pub fn get_value(&self) -> &Email { &self.value }
    pub fn get_type_name() -> String {
        "EmailValue".to_string()
    }
    pub fn get_type(&self) -> String {
        Self::get_type_name()
    }
}

#[cfg(test)]
mod tests {
    use serde_email::Email;
    use crate::core::types::email_value::{get_value_generator, EmailValue};
    use crate::core::types::ValueType;
    use super::ValueBuilder;
    #[test]
    fn test_string_creation_success() {
        const RAW_VALUE: &str = "test@example.com";
        let expected_result = Email::from_str(RAW_VALUE).unwrap();
        let builder = EmailValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().get_value(), expected_result);
    }
    #[test]
    fn test_string_creation_failure() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "testexample.com";
        let builder = EmailValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_string_creation_failure_2() {
        let builder = EmailValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_get_value_generator() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "test@example.com";
        let expected_result = Email::from_str(RAW_VALUE).unwrap();
        let generator = get_value_generator();
        let value = generator(RAW_VALUE.to_string()).unwrap();
        match value.as_ref().get_value() {
            ValueType::Email(value) => assert_eq!(*value.get_value(), expected_result),
            _ => assert!(false),
        };
    }
}