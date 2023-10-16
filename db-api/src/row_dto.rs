use std::rc::Rc;
use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use core::row::Row;
use core::types::CellValue;

use core::types::ValueType;
use core::types::int_value::IntValue;
use core::types::char_value::CharValue;
use core::types::date_value::DateValue;
use core::types::picture_value::PictureValue;
use core::types::real_value::RealValue;

use crate::char_value_dto::CharValueDTO;
use crate::date_value_dto::DateValueDTO;

use crate::Encoder;
use crate::Decoder;
use crate::envelope::Envelope;
use crate::int_value_dto::IntValueDTO;
use crate::picture_value_dto::PictureValueDTO;
use crate::real_value_dto::RealValueDTO;
use crate::string_value_dto::StringValueDTO;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RowDTO {
    values: Vec<Envelope>,
}

impl From<Rc<Row<dyn CellValue>>> for RowDTO {
    fn from(value: Rc<Row<dyn CellValue>>) -> Self {
        let row_values = value.get_values();
        let mut values = Vec::<Envelope>::with_capacity(row_values.len());
        row_values.iter().for_each(|value| {
            let wrapper = match value.get_value() {
                ValueType::Int(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), IntValueDTO::new(v).encode().as_slice())
                }
                ValueType::Str(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), StringValueDTO::new(v).encode().as_slice())
                }
                ValueType::Real(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), RealValueDTO::new(v).encode().as_slice())
                }
                ValueType::Pic(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), PictureValueDTO::new(v).encode().as_slice())
                }
                ValueType::Char(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), CharValueDTO::new(v).encode().as_slice())
                }
                ValueType::Date(v) => {
                    let ty = v.get_type();
                    Envelope::new(ty.as_str(), DateValueDTO::new(v).encode().as_slice())
                }
                ValueType::Email(_v) => todo!(),
            };
            values.push(wrapper);
        });
        Self { values }
    }
}
impl From<RowDTO> for Row<dyn CellValue> {
    fn from(value: RowDTO) -> Self {
        let mut row_values = Vec::with_capacity(value.values.len());
        value.values.iter().for_each(|wrapper| {
            let ty = wrapper.get_type();
            let value: Rc<dyn CellValue> = if ty == IntValue::get_type_name() {
                Rc::new(IntValueDTO::decode(wrapper.get_data().to_vec()).value)
            } else if ty == CharValue::get_type_name() {
                Rc::new(CharValueDTO::decode(wrapper.get_data().to_vec()).value)
            } else if ty == DateValue::get_type_name() {
                Rc::new(DateValueDTO::decode(wrapper.get_data().to_vec()).value)
            } else if ty == PictureValue::get_type_name() {
                Rc::new(PictureValueDTO::decode(wrapper.get_data().to_vec()).value)
            } else if ty == RealValue::get_type_name() {
                Rc::new(RealValueDTO::decode(wrapper.get_data().to_vec()).value)
            } else {
                Rc::new(StringValueDTO::decode(wrapper.get_data().to_vec()).value)
            };
            row_values.push(value);
        });
        Row::<dyn CellValue>::new(row_values)
    }
}

impl RowDTO {
    pub fn new(values: Vec<Envelope>) -> Self {
        Self { values }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();


        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("values");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for wrapper in self.values.iter() {
            let data = wrapper.encode();
            writer.write_blob(data.as_slice()).unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
    pub fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut values = Vec::<Envelope>::with_capacity(elements.capacity());
        for element in elements {
            let data = element.as_blob().unwrap();
            values.push(Envelope::decode(data));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        Self { values }
    }
}