use ion_rs;
use core::types::ValueBuilder;
use core::types::email_value::EmailValue;

use ion_rs::IonWriter;
use ion_rs::IonReader;

#[derive(Debug, PartialEq, Clone)]
pub struct EmailValueDTO {
    pub value: EmailValue,
}

impl EmailValueDTO {
    pub fn new(value: EmailValue) -> EmailValueDTO {
        Self { value }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();


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
        let ans = binary_user_reader.read_string().unwrap().to_string();
        let value = EmailValue::builder()
            .with_raw_value(ans)
            .build()
            .unwrap();
        EmailValueDTO::new(value)
    }
}