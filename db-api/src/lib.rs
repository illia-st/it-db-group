pub trait Decoder {
    fn decode(data: &[u8]) -> Self;
}
pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}
pub mod db;
pub mod row_dto;
pub mod table;
pub mod envelope;

pub mod int_value_dto;
pub mod real_value_dto;
pub mod char_value_dto;
pub mod string_value_dto;
pub mod picture_value_dto;
pub mod date_value_dto;
pub mod scheme_dto;