use std::rc::Rc;
use std::sync::Arc;
use chrono::{DateTime, NaiveDateTime, Utc};
use super::{CellValue, ValueType};
use super::ValueBuilder;
use value_generator::ValueGenerator;


#[derive(Clone, Debug, Default, PartialEq, ValueGenerator)]
pub struct DateValue {
    value: DateTime<Utc>,
}
impl DateValue {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self { value }
    }
    pub fn builder() -> CharValueBuilder {
        CharValueBuilder::default()
    }
    pub fn get_value(&self) -> DateTime<Utc> {
        self.value
    }
    pub fn get_type_name() -> String {
        "DateValue".to_string()
    }
    pub fn get_type(&self) -> String {
        Self::get_type_name()
    }
}

impl CellValue for DateValue {
    fn get_value(&self) -> ValueType {
        ValueType::Date(self.clone())
    }
}
#[derive(Default)]
pub struct CharValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for CharValueBuilder {
    type Value = DateValue;
    type RowValueType = DateTime<Utc>;
    fn validate(&self) -> Result<Self::RowValueType, String> {
        // TODO: think about removing duplication here
        match &self.row_value {
            Some(value) => {
                let trimmed_value = value.trim();
                // "Sep 18, 2013 07:49:07.000000000 EEST"
                // "Dec  5, 2004 21:16:24.317453000 UTC"
                if let Ok(res) = NaiveDateTime::parse_from_str(trimmed_value, "%b %d, %Y %H:%M:%S.%f %Z") {
                    return Ok(res.and_utc());
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
            Ok(value) => Ok(DateValue::new(value)),
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
    use chrono::{DateTime, NaiveDateTime, Utc};
    use crate::core::types::date_value::{DateValue, get_value_generator};
    use crate::core::types::ValueType;
    use super::ValueBuilder;
    #[test]
    fn test_date_creation_success() {
        const RAW_VALUE: &str = "Dec  5, 2004 21:16:24.317453000 EET";
        let expected_result: DateTime<Utc> = NaiveDateTime::parse_from_str(RAW_VALUE, "%b %d, %Y %H:%M:%S.%f %Z").unwrap().and_utc();
        let builder = DateValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), expected_result);
    }
    #[test]
    fn test_char_creation_failure() {
        const RAW_VALUE: &str = "5, 2004 21:16:24.317453000 EET";
        // const EXPECTED_RESULT: DateTime<Utc> = NaiveDateTime::parse_from_str(RAW_VALUE, "%b %d, %Y %H:%M:%S.%f %Z").unwrap().and_utc();
        let builder = DateValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_get_value_generator() {
        const RAW_VALUE: &str = "Dec  5, 2004 21:16:24.317453000 EET";
        let generator = get_value_generator();
        let expected_result: DateTime<Utc> = NaiveDateTime::parse_from_str(RAW_VALUE, "%b %d, %Y %H:%M:%S.%f %Z").unwrap().and_utc();
        let value = generator(RAW_VALUE.to_string()).unwrap();
        match value.as_ref().get_value() {
            ValueType::Date(value) => assert_eq!(value.get_value(), expected_result),
            _ => assert!(false),
        };
    }
}