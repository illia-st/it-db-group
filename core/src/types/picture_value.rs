use std::rc::Rc;
use std::sync::Arc;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use value_generator::ValueGenerator;
use crate::types::{CellValue, ValueType};
use super::ValueBuilder;

#[derive(Clone, Debug, Default, PartialEq, ValueGenerator)]
pub struct PictureValue {
    value: DynamicImage,
}
impl CellValue for PictureValue {
    fn get_value(&self) -> ValueType {
        ValueType::Pic(self.clone())
    }
}
#[derive(Default)]
pub struct PictureValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for PictureValueBuilder {
    type Value = PictureValue;
    type RowValueType = DynamicImage;
    fn validate(&self) -> Result<Self::RowValueType, String> {
        match &self.row_value {
            Some(value) => {
                let image = match ImageReader::open(value.trim()) {
                    Ok(buf) => buf,
                    Err(err) => return Err(format!("error occurred while trying to read image: {}", err))
                };
                match image.decode() {
                    Ok(image) => Ok(image),
                    Err(err) => Err(format!("error occurred while trying to decode image: {}", err))
                }
            },
            None => {
                Err("the value is expected to be set up".to_string())
            }
        }
    }

    fn build(self) -> Result<Self::Value, String> {
        match self.validate() {
            Ok(value) => Ok(PictureValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

impl PictureValue {
    fn new(value: DynamicImage) -> Self {
        Self { value }
    }
    pub fn builder() -> PictureValueBuilder {
        PictureValueBuilder::default()
    }
    pub fn get_value(&self) -> &DynamicImage {
        &self.value
    }
    pub fn get_type_name() -> String {
        "PictureValue".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::picture_value::{get_value_generator, PictureValue};
    use super::ValueBuilder;
    use crate::test_resources;
    use image::DynamicImage;
    use image::io::Reader as ImageReader;
    use crate::types::ValueType;

    #[test]
    fn test_picture_creation_success_happy_cat_jpg() {
        const RAW_VALUE: &str = test_resources!("happy_cat.jpg");
        let expected_result: DynamicImage =
            ImageReader::open(test_resources!("happy_cat.jpg"))
                .unwrap()
                .decode()
                .unwrap();
        let builder = PictureValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value().as_bytes(), expected_result.as_bytes());
    }
    #[test]
    fn test_int_creation_success_sad_cat_png() {
        const RAW_VALUE: &str = test_resources!("sad_cat.png");
        let expected_result: DynamicImage =
            ImageReader::open(test_resources!("sad_cat.png"))
                .unwrap()
                .decode()
                .unwrap();
        let builder = PictureValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value().as_bytes(), expected_result.as_bytes());
    }
    #[test]
    fn test_int_creation_failure_non_existing_cat() {
        const RAW_VALUE: &str = test_resources!("non-existing.png");
        let builder = PictureValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_int_creation_failure_3() {
        let builder = PictureValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
    #[test]
    fn test_get_value_generator() {
        const RAW_VALUE: &str = test_resources!("happy_cat.jpg");
        let expected_result: DynamicImage =
            ImageReader::open(RAW_VALUE)
                .unwrap()
                .decode()
                .unwrap();
        let generator = get_value_generator();
        let value = generator(RAW_VALUE.to_string()).unwrap();
        match value.as_ref().get_value() {
            ValueType::Pic(value) => {
                assert_eq!(value.get_value().as_bytes(), expected_result.as_bytes())
            },
            _ => assert!(false),
        };
    }
}