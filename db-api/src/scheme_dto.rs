use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use core::scheme::Scheme;
use core::types::CellValue;
use core::types::SUPPORTED_TYPES;
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SchemeDTO {
    types: Vec<String>,
    columns: Vec<String>,
}

impl From<SchemeDTO> for Scheme<dyn CellValue> {
    fn from(value: SchemeDTO) -> Self {
        let mut value_generators = Vec::with_capacity(value.types.len());
        value.types.iter().for_each(|ty| {
            value_generators.push(SUPPORTED_TYPES.get(ty.as_str()).unwrap().clone());
        });
        Scheme::new(value.types, value.columns, value_generators)
    }
}

impl From<Scheme<dyn CellValue>> for SchemeDTO {
    fn from(value: Scheme<dyn CellValue>) -> Self {
        Self {
            types: value.get_types(),
            columns: value.get_columns(),
        }
    }
}

impl SchemeDTO {
    pub fn new(types: Vec<String>, columns: Vec<String>) -> Self {
        Self {
            types,
            columns
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();


        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("types");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for ty in self.types.iter() {
            writer.write_string(ty).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("columns");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for column in self.columns.iter() {
            writer.write_string(column).unwrap();
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
        let mut types = Vec::<String>::with_capacity(elements.capacity());
        for element in elements {
            let ty = element.as_string().unwrap().to_string();
            types.push(ty);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut columns = Vec::<String>::with_capacity(elements.capacity());
        for element in elements {
            let column = element.as_string().unwrap().to_string();
            columns.push(column);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        Self {
            types,
            columns,
        }
    }
}