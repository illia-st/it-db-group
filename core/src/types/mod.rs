#![allow(clippy::type_complexity)]
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;
use crate::types::char_value::CharValue;
use crate::types::int_value::IntValue;
use crate::types::picture_value::PictureValue;
use crate::types::real_value::RealValue;
use crate::types::string_value::StringValue;
use std::collections::HashMap;
use lazy_static::lazy_static;

// TODO: think about how we can refuse from using enum bcs smells like bad design decision
#[derive(PartialEq)]
pub enum ValueType {
    Int(IntValue),
    Str(StringValue),
    Real(RealValue),
    Pic(PictureValue),
    Char(CharValue)
}
pub trait CellValue: Debug {
    fn get_value(&self) -> ValueType;
}

lazy_static! {
    static ref SUPPORTED_TYPES: HashMap<String, Arc<fn(String) -> Result<Rc<dyn CellValue>, String>>> = {
        let mut supported_types = HashMap::new();
        supported_types.insert(IntValue::get_type_name(), crate::types::int_value::get_value_generator());
        supported_types.insert(CharValue::get_type_name(), crate::types::char_value::get_value_generator());
        supported_types.insert(PictureValue::get_type_name(), crate::types::picture_value::get_value_generator());
        supported_types.insert(RealValue::get_type_name(), crate::types::real_value::get_value_generator());
        supported_types.insert(StringValue::get_type_name(), crate::types::string_value::get_value_generator());
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
