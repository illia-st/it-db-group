use ion_rs;
use core::types::real_value::RealValue;
use ion_rs::{IonWriter, TextWriterBuilder};
use ion_rs::element::writer::TextKind;
use ion_rs::IonReader;

#[derive(Debug, PartialEq, Clone)]
pub struct RealValueDTO {
    pub value: RealValue,
}

impl RealValueDTO {
    pub fn new(value: RealValue) -> RealValueDTO {
        Self { value }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let text_writer_builder = TextWriterBuilder::new(TextKind::Compact);
        let mut writer = text_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("value");
        writer.write_f64(self.value.get_value()).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
    pub fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let value = binary_user_reader.read_f64().unwrap();
        RealValueDTO::new(RealValue::new(value))
    }
}