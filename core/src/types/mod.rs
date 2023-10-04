use std::fmt::Debug;
use crate::types::char_value::CharValue;
use crate::types::int_value::IntValue;
use crate::types::picture_value::PictureValue;
use crate::types::real_value::RealValue;
use crate::types::string_value::StringValue;

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
