use ion_rs;
use core::types::string_value::StringValue;
use ion_rs::{IonWriter, TextWriterBuilder};
use ion_rs::element::writer::TextKind;
use ion_rs::IonReader;

#[derive(Debug, PartialEq, Clone)]
pub struct StringValueDTO {
    pub value: StringValue,
}

impl StringValueDTO {
    pub fn new(value: StringValue) -> StringValueDTO {
        Self { value }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let text_writer_builder = TextWriterBuilder::new(TextKind::Compact);
        let mut writer = text_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("value");
        writer.write_string(self.value.get_value()).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
    pub fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let value = binding.text();
        StringValueDTO::new(StringValue::new(value.to_owned()))
    }
}