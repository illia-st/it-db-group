use crate::types::{CellValue, ValueType};
use super::ValueBuilder;
#[derive(Clone, Default)]
pub struct IntValue {
    value: i64,
}

impl CellValue for IntValue {
    fn get_value(&self) -> ValueType {
        ValueType::Int(self.clone())
    }
}
#[derive(Default)]
pub struct IntValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for IntValueBuilder {
    type Value = IntValue;
    type RowValueType = i64;
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
            Ok(value) => Ok(IntValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

impl IntValue {
    fn new(value: i64) -> Self {
        Self {
            value
        }
    }
    pub fn builder() -> IntValueBuilder {
        IntValueBuilder::default()
    }
    pub fn get_value(&self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::types::int_value::IntValue;
    use super::ValueBuilder;
    #[test]
    fn test_int_creation_success_1() {
        const RAW_VALUE: &str = "1";
        const EXPECTED_RESULT: i64 = 1;
        let builder = IntValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_int_creation_success_21() {
        const RAW_VALUE: &str = " 21  ";
        const EXPECTED_RESULT: i64 = 21;
        let builder = IntValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_int_creation_failure_1() {
        // https://www.youtube.com/watch?v=hB-WHw6uMWg
        const RAW_VALUE: &str = "throw away your television";
        let builder = IntValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_int_creation_failure_2() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "21st Century";
        let builder = IntValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_int_creation_failure_3() {
        let builder = IntValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
}