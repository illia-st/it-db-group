use std::rc::Rc;
use std::sync::Arc;
use super::{CellValue, ValueType};
use super::ValueBuilder;
use value_generator::ValueGenerator;


#[derive(Clone, Debug, Default, PartialEq, ValueGenerator)]
pub struct CharValue {
    value: char,
}
impl CharValue {
    fn new(value: char) -> Self {
        Self { value }
    }
    pub fn builder() -> CharValueBuilder {
        CharValueBuilder::default()
    }
    pub fn get_value(&self) -> char {
        self.value
    }
    pub fn get_type_name() -> String {
        "CharValue".to_string()
    }

    pub fn get_type(&self) -> String {
        Self::get_type_name()
    }
}

impl CellValue for CharValue {
    fn get_value(&self) -> ValueType {
        ValueType::Char(self.clone())
    }
}
#[derive(Default)]
pub struct CharValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for CharValueBuilder {
    type Value = CharValue;
    type RowValueType = char;
    fn validate(&self) -> Result<Self::RowValueType, String> {
        // TODO: think about removing duplication here
        match &self.row_value {
            Some(value) => {
                let trimmed_value = value.trim();
                if let Ok(res) = trimmed_value.parse::<Self::RowValueType>() {
                    return Ok(res);
                };
                Err(format!("validation has failed: {}", trimmed_value))
            },
            None => {
                Err("the value is expected to be set up".to_string())
            }
        }
    }

    fn build(self) -> Result<Self::Value, String> {
        match self.validate() {
            Ok(value) => Ok(CharValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::char_value::{CharValue, get_value_generator};
    use crate::core::types::ValueType;
    use super::ValueBuilder;
    #[test]
    fn test_char_creation_success_1() {
        const RAW_VALUE: &str = "a";
        const EXPECTED_RESULT: char = 'a';
        let builder = CharValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_char_creation_success_21() {
        const RAW_VALUE: &str = "2";
        const EXPECTED_RESULT: char = '2';
        let builder = CharValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_char_creation_failure_1() {
        // https://www.youtube.com/watch?v=hB-WHw6uMWg
        const RAW_VALUE: &str = "throw away your television";
        let builder = CharValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_char_creation_failure_2() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "21st Century";
        let builder = CharValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }

    #[test]
    fn test_char_creation_failure_3() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = " 21 ";
        let builder = CharValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_char_creation_failure_4() {
        let builder = CharValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_get_value_generator() {
        let generator = get_value_generator();
        let value = generator("a".to_string()).unwrap();
        match value.as_ref().get_value() {
            ValueType::Char(value) => assert_eq!(value.get_value(), 'a'),
            _ => assert!(false),
        };
    }
}