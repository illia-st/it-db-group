use std::rc::Rc;
use std::sync::Arc;
use value_generator::ValueGenerator;
use crate::core::types::{CellValue, ValueType};
use super::ValueBuilder;

#[derive(Clone, Debug, Default, PartialEq, ValueGenerator)]
pub struct RealValue {
    value: f64,
}

impl CellValue for RealValue {
    fn get_value(&self) -> ValueType {
        ValueType::Real(self.clone())
    }
}
#[derive(Default)]
pub struct RealValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for RealValueBuilder {
    type Value = RealValue;
    type RowValueType = f64;
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
            Ok(value) => Ok(RealValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

impl RealValue {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn builder() -> RealValueBuilder {
        RealValueBuilder::default()
    }
    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_type_name() -> String {
        "RealValue".to_string()
    }
    pub fn get_type(&self) -> String {
        Self::get_type_name()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::real_value::{get_value_generator, RealValue};
    use crate::core::types::ValueType;
    use super::ValueBuilder;
    #[test]
    fn test_real_creation_success_1() {
        const RAW_VALUE: &str = "1.54222";
        const EXPECTED_RESULT: f64 = 1.54222;
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_real_creation_success_2() {
        const RAW_VALUE: &str = "7746263.1234542";
        const EXPECTED_RESULT: f64 = 7746263.1234542;
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_real_creation_success_3() {
        const RAW_VALUE: &str = "  -12462.1542";
        const EXPECTED_RESULT: f64 = -12462.1542;
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_real_creation_success_4() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = " 21 ";
        const EXPECTED_RESULT: f64 = 21.;
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_real_creation_failure_1() {
        // https://www.youtube.com/watch?v=hB-WHw6uMWg
        const RAW_VALUE: &str = "throw away your television";
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_real_creation_failure_2() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "21st Century";
        let builder = RealValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_real_creation_failure_3() {
        let builder = RealValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_get_value_generator() {
        let generator = get_value_generator();
        let value = generator("1.23".to_string()).unwrap();
        match value.as_ref().get_value() {
            ValueType::Real(value) => assert_eq!(value.get_value(), 1.23),
            _ => assert!(false),
        };
    }
}