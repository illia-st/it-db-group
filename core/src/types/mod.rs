pub trait ValueBuilder {
    type Value;
    type RowValueType;
    fn validate(&self) -> Result<Self::RowValueType, String>;
    fn build(self) -> Result<Self::Value, String>;
    fn with_raw_value(self, raw_value: String) -> Self;
}

pub mod int_value;
pub mod string_value;
pub mod real_value;
pub mod picture_value;
pub mod char_value;


// use crate::ValueBuilder;
//
// pub struct RealValue {
//     value: Option<f64>,
//     raw_value: String
// }
//
// impl RealValue {
//     pub fn builder() -> RealValueBuilder {
//         RealValueBuilder::default()
//     }
// }
//
// #[derive(Default)]
// pub struct RealValueBuilder {
//     row_value: Option<String>,
// }
// impl ValueBuilder for RealValueBuilder {
//     fn validate(&self) -> bool {
//         todo!()
//     }
//
//     fn build(self) -> Self {
//         todo!()
//     }
//
//     fn with_raw_value(mut self, raw_value: String) -> Self {
//         self.row_value = Some(raw_value);
//         self
//     }
// }
//
// pub struct CharValue {
//     value: Option<char>,
//     raw_value: String
// }
//
// impl ValueBuilder for IntValueBuilder {
//     fn validate(&self) -> bool {
//         todo!()
//     }
//
//     fn build(self) -> Self {
//         todo!()
//     }
//
//     fn with_raw_value(mut self, raw_value: String) -> Self {
//         self.row_value = Some(raw_value);
//         self
//     }
// }
// pub struct StringValue {
//     value: Option<String>,
//     raw_value: String
// }
//
// impl ValueBuilder for StringValue {
//     fn validate(&self) -> bool {
//         todo!()
//     }
//
//     fn build(self) -> Self {
//         todo!()
//     }
//
//     fn with_raw_value(mut self, raw_value: String) -> Self {
//         self.row_value = Some(raw_value);
//         self
//     }
// }
//
// pub struct PictureColumn {
//     value: Option<Vec<u8>>,
//     raw_value: String
// }
//
// impl ValueBuilder for PictureColumn {
//     fn validate(&self) -> bool {
//         todo!()
//     }
//
//     fn build(self) -> Self {
//         todo!()
//     }
//
//     fn with_raw_value(mut self, raw_value: String) -> Self {
//         self.row_value = Some(raw_value);
//         self
//     }
// }
// #[cfg(test)]
// mod tests {
//     use crate::types::IntValue;
//     use crate::Value;
//
//     #[test]
//     fn test_int_validation() {
//         let int_value = IntValue::new("1".to_string())
//             .validate()
//     }
// }