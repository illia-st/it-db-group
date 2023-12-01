#![allow(clippy::type_complexity)]
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;
use super::types::char_value::CharValue;
use super::types::int_value::IntValue;
use super::types::picture_value::PictureValue;
use super::types::real_value::RealValue;
use super::types::string_value::StringValue;
use std::collections::HashMap;
use lazy_static::lazy_static;
use super::types::date_value::DateValue;
use super::types::email_value::EmailValue;

// TODO: think about how we can refuse from using enum bcs smells like bad design decision
#[derive(PartialEq)]
pub enum ValueType {
    Int(IntValue),
    Str(StringValue),
    Real(RealValue),
    Pic(PictureValue),
    Char(CharValue),
    Date(DateValue),
    Email(EmailValue),
}
pub trait CellValue: Debug {
    fn get_value(&self) -> ValueType;
}

lazy_static! {
    pub static ref SUPPORTED_TYPES: HashMap<String, Arc<fn(String) -> Result<Rc<dyn CellValue>, String>>> = {
        let mut supported_types = HashMap::new();
        supported_types.insert(IntValue::get_type_name(), int_value::get_value_generator());
        supported_types.insert(CharValue::get_type_name(), char_value::get_value_generator());
        supported_types.insert(PictureValue::get_type_name(), picture_value::get_value_generator());
        supported_types.insert(RealValue::get_type_name(), real_value::get_value_generator());
        supported_types.insert(StringValue::get_type_name(), string_value::get_value_generator());
        supported_types.insert(DateValue::get_type_name(), date_value::get_value_generator());
        supported_types.insert(EmailValue::get_type_name(), email_value::get_value_generator());
        supported_types
    };
}

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
pub mod date_value;
pub mod email_value;
