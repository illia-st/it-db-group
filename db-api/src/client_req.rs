use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;

#[derive(Debug, PartialEq, Clone)]
pub struct ClientRequest {
    pub command_type: String,
    // arguments
    pub database_path: Option<String>,
    pub database_name: Option<String>,
    pub save: Option<bool>,
}

impl ClientRequest {
    pub fn new(command_type: String, database_path: Option<String>, database_name: Option<String>, save: Option<bool>) -> ClientRequest {
        Self {
            command_type,
            database_path,
            database_name,
            save,
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("command_type");
        writer.write_string(&self.command_type).unwrap();

        writer.set_field_name("database_path");
        match &self.database_path {
            Some(path) => writer.write_string(path).unwrap(),
            None => writer.write_null(ion_rs::IonType::String).unwrap(),
        }

        writer.set_field_name("database_name");
        match &self.database_name {
            Some(name) => writer.write_string(name).unwrap(),
            None => writer.write_null(ion_rs::IonType::String).unwrap(),
        }

        writer.set_field_name("save");
        match &self.save {
            Some(save) => writer.write_bool(*save).unwrap(),
            None => writer.write_null(ion_rs::IonType::Bool).unwrap(),
        }

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
    pub fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let command_type = binary_user_reader.read_string().unwrap().to_string();

        binary_user_reader.next().unwrap();
        let database_path = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                Some(binary_user_reader.read_string().unwrap().to_string())
            },
            ion_rs::StreamItem::Null(_) => None,
            ion_rs::StreamItem::Nothing => todo!(),
        };

        binary_user_reader.next().unwrap();
        let database_name = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                Some(binary_user_reader.read_string().unwrap().to_string())
            },
            ion_rs::StreamItem::Null(_) => None,
            ion_rs::StreamItem::Nothing => todo!(),
        };

        binary_user_reader.next().unwrap();
        let save = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                Some(binary_user_reader.read_bool().unwrap())
            },
            ion_rs::StreamItem::Null(_) => None,
            ion_rs::StreamItem::Nothing => todo!(),
        };
        Self::new(
            command_type,
            database_path,
            database_name,
            save,
        )

    }
}