
use std::rc::Rc;
use ion_rs;
use ion_rs::{IonWriter, TextWriterBuilder};
use ion_rs::element::reader::ElementReader;
use ion_rs::element::writer::TextKind;
use ion_rs::IonReader;
use crate::core::table::Table;
use crate::core::scheme::Scheme;
use crate::core::types::CellValue;
use crate::core::row::Row;
use super::row_dto::RowDTO;
use super::scheme_dto::SchemeDTO;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableDTO {
    pub name: String,
    pub scheme: SchemeDTO,
    pub rows: Vec<RowDTO>,
}

impl From<TableDTO> for Table {
    fn from(value: TableDTO) -> Self {
        let schema: Scheme<dyn CellValue> = value.scheme.into();
        let table = Table::new(value.name, schema);
        let mut rows: Vec<Rc<Row<dyn CellValue>>> = Vec::with_capacity(value.rows.len());
        value.rows.into_iter().for_each(|row| {
            let new_row = Row::<dyn CellValue>::from(row);
            rows.push(Rc::new(new_row));
        });
        table.set_rows(rows);
        table
    }
}

impl From<Table> for TableDTO {
    fn from(value: Table) -> Self {
        let core_rows = value.rows.take();
        let name = value.name;
        let scheme: SchemeDTO = value.scheme.into();
        let mut rows = Vec::<RowDTO>::with_capacity(core_rows.len());
        for row in core_rows.into_iter() {
            rows.push(RowDTO::from(row));
        }
        Self {
            name,
            scheme,
            rows,
        }
    }
}

impl TableDTO {
    pub fn new(name: String, scheme: SchemeDTO, rows: Vec<RowDTO>) -> Self {
        Self {
            name,
            scheme,
            rows
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let text_writer_builder = TextWriterBuilder::new(TextKind::Compact);
        let mut writer = text_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("scheme");
        writer.write_blob(&self.scheme.encode()).unwrap();

        writer.set_field_name("rows");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for row in self.rows.iter() {
            let data = row.encode();
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
        let binding = binary_user_reader.read_string().unwrap();
        let name = binding.text();

        binary_user_reader.next().unwrap();
        let scheme = SchemeDTO::decode(binary_user_reader.read_blob().unwrap().as_slice().to_vec());

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut rows = Vec::<RowDTO>::with_capacity(elements.capacity());
        for element in elements {
            let data = element.as_blob().unwrap();
            rows.push(RowDTO::decode(data.to_vec()));
        }

        binary_user_reader.step_out().unwrap();

        Self {
            name: name.to_owned(),
            scheme,
            rows,
        }
    }
}