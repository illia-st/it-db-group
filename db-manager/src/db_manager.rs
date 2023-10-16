use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;

use core::db::Database;
use core::types::CellValue;
use core::scheme::Scheme;
use core::row::Row;
use core::types::SUPPORTED_TYPES;
use core::table::Table;
use db_api::db::DatabaseDTO;

// Can operate with one db-manager at the time
#[derive(Debug)]
pub struct DatabaseManager {
    #[allow(clippy::type_complexity)]
    supported_types: HashMap<String, Arc<fn(String) -> Result<Rc<dyn CellValue>, String>>>,
    database: RefCell<Option<Database>>,
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseManager {
    // creating a database manager
    pub fn new() -> Self {
        Self {
            supported_types: SUPPORTED_TYPES.clone(),
            database: RefCell::new(None),
        }
    }
    pub fn create_db(&self, name: &str, location: &str) -> Result<(), String> {
        let _ = self.close_db(true);
        // check if such a dir is existing
        if let Ok(metadata) = fs::metadata(location) {
            if !metadata.is_dir() {
                return Err("provided path points to the file or symlink".to_string());
            }
        }
        // create a file for database
        match File::create(format!("{}/{}", location, name)) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a file: {err}"))
        }
        // build db-manager using Database::builder()
        let database = Database::builder()
            .with_location(location)
            .with_name(name)
            .build()
            .unwrap();
        *self.database.borrow_mut().deref_mut() = Some(database);
        Ok(())
    }

    pub fn read_db_from_directory(&self, dir: &str, file_name: &str) -> Result<(), String> {
        let location = &format!("{}/{}", dir, file_name);
        // need to close the previous one
        let _ = self.close_db(true);
        // check if provided location is a dir
        match fs::metadata(location) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err("provided path points to the dir or symlink".to_string());
                }
            },
            Err(err) => return Err(format!("couldn't read the file {}: {}", location, err))
        };
        // read file location/table using amazon ion
        let database = match fs::read(location) {
            Ok(database) => database,
            Err(err) => {
                let err_string = format!("The error is occurred while trying to read tables: {}", err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            }
        };
        let db_dto = DatabaseDTO::decode(database);
        self.database.borrow_mut().replace(Database::from(db_dto));
        Ok(())
    }
    
    pub fn create_table(&self, table_name: &str, columns: Vec<&str>, data_types: Vec<&str>) -> Result<(), String> {
        // 1) check if the table already exists
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        if columns.len() != data_types.len() {
            return Err("Different number of columns and data types".to_string());
        }
        #[allow(clippy::type_complexity)]
        let mut value_generators: Vec<Arc<fn(String) -> Result<Rc<dyn CellValue>, String>>> = Vec::with_capacity(data_types.len());
        let mut new_columns = Vec::with_capacity(columns.len());
        let mut types = Vec::with_capacity(data_types.len());
        for (data_type, column_name) in data_types.iter().zip(columns) {
            match self.supported_types.get(*data_type) {
                Some(value_generator) => value_generators.push(value_generator.clone()),
                None => return Err(format!("No such supported data type: {}", data_type))
            }
            new_columns.push(column_name.to_string());
            types.push(data_type.to_string());
        }
        let scheme = Scheme::new(types, new_columns, value_generators);
        let table = match Table::builder()
            .with_name(table_name.to_string())
            .with_scheme(scheme)
            .build() {
            Ok(table) => table,
            Err(err) => return Err(err)
        };
        let mut db = self.database.borrow_mut();
        let unwrapped_db = db.as_mut().unwrap();
        unwrapped_db.get_tables_mut().insert(table_name.to_string(), table);
        Ok(())
    }
    pub fn delete_table(&self, table_name: &str) -> Result<(), String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let mut db = self.database.borrow_mut();
        let db_unwrapped = db.as_mut().unwrap();
        let mut tables = db_unwrapped.get_tables_mut();
        match tables.deref_mut().remove(table_name) {
            Some(_) => Ok(()),
            None => Err(format!("There is no table with name {}", table_name))
        }
    }
    pub fn add_row(&self, table_name: &str, raw_values: &str) -> Result<(), String>{
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let db = self.database.borrow();
        let db_unwrapped = db.as_ref().unwrap();
        let res = match db_unwrapped.get_tables_mut().get_mut(table_name) {
            Some(table) => {
                let scheme = table.get_scheme();
                let split_values = raw_values
                    .split(';')
                    .map(|value| value.trim().to_string());

                let mut row_values = Vec::default();
                for (generator, raw_value) in scheme.get_validators().iter().zip(split_values) {
                    match generator(raw_value) {
                        Ok(value) => row_values.push(value),
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                let new_row = Row::new(row_values);
                log::debug!("Added row into table {} with values {:?}", table_name, new_row);
                table.add_row(new_row);
                Ok(())
            },
            None => Err(format!("There is no table with name {}", table_name))
        };
        res
    }

    pub fn add_row_to_the_table(table: &Table, raw_values: Vec<String>){
        let scheme = table.get_scheme();

        let mut row_values = Vec::default();
        for (generator, raw_value) in scheme.get_validators().iter().zip(raw_values) {
            match generator(raw_value) {
                Ok(value) => row_values.push(value),
                Err(_) => panic!()
            }
        }
        let new_row = Row::new(row_values);
        table.add_row(new_row);
    }

    pub fn delete_row(&self, table_name: &str, index: u64) -> Result<(), String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let db = self.database.borrow();
        let db_unwrapped = db.as_ref().unwrap();
        let res = match db_unwrapped.get_tables_mut().get_mut(table_name) {
            Some(table) => table.erase(index),
            None => {
                let err_string = format!("There is no table {} in {}", table_name, db_unwrapped.get_name());
                log::error!("{}", err_string.as_str());
                Err(err_string)
            }
        };
        res
    }
    pub fn close_db(&self, save: bool) -> Result<(), String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let db = self.database.take().unwrap();
        let res = if save {
            let db_dto: DatabaseDTO = db.into();
            let location = &format!("{}/{}", db_dto.location, db_dto.name);
            let fd = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(location);
            let mut file = match fd {
                Ok(file) => file,
                Err(err) => return Err(format!("couldn't open the file to save {}: {}", location, err))
            };
            match file.write_all(db_dto.encode().as_slice()) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("couldn't write to the file to save {}: {}", location, err)),
            }
        } else {
            Ok(())
        };
        res
    }
    pub fn delete_db(&self, dir: &str, file_name: &str) -> Result<(), String> {
        // TODO: it will be nice to check if the provided location actually is a db but who cares?
        let location = &format!("{}/{}", dir, file_name);
        match fs::remove_file(location) {
            Ok(()) => {
                log::debug!("Database in {} has been removed", location);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete database in {}: {}", location, err);
                log::error!("{}", err_string.as_str());
                Err(err_string)
            },
        }
    }
    pub fn get_table(&self, table_name: &str) -> Result<Table, String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        match self.database.borrow().as_ref().unwrap().get_tables_mut().get_mut(table_name) {
            Some(table) => Ok(table.clone()),
            None => Err(format!("there is no table with name {}", table_name))
        }
    }
    pub fn get_existing_table_names(&self) -> Result<Vec<String>, String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        Ok(self.database.borrow().as_ref().unwrap().get_tables().keys().cloned().collect())
    }
    pub fn db_is_opened(&self) -> bool {
        self.database.borrow().is_some()
    }
    pub fn get_table_list(&self) -> Vec<String> {
        self.database.borrow().as_ref().unwrap().get_tables().keys().cloned().collect::<Vec<String>>()
    }

    pub fn join(&self, lhs_table_name: &str, rhs_table_name: &str, _column: &str) -> Result<Table, String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let lhs = match self.get_table(lhs_table_name) {
            Ok(lhs) => lhs,
            Err(err) => return Err(err)
        };
        let rhs = match self.get_table(rhs_table_name) {
            Ok(rhs) => rhs,
            Err(err) => return Err(err)
        };
        
        let lhs_columns = lhs.scheme.get_columns();
        let rhs_columns = rhs.scheme.get_columns();
        if !lhs_columns.contains(&column.to_owned()) || !rhs_columns.contains(&column.to_owned()) {
            return Err("There is no such column in at least one of the tables".to_string())
        }
        for lhs_column in &lhs_columns {
            if lhs_column != column && rhs_columns.contains(&lhs_column) {
                return Err("There are some other columns, that have equal names".to_string());
            }
        }

        let lhs_column_index = lhs.get_scheme().get_columns().iter().position(|n| n == column).unwrap();
        let rhs_column_index = rhs.get_scheme().get_columns().iter().position(|n| n == column).unwrap();

        let lhs_range = 0..lhs.get_scheme().get_columns().len();
        let rhs_range = 0..rhs.get_scheme().get_columns().len();

        let lhs_scheme = lhs.get_scheme();
        let rhs_scheme = rhs.get_scheme();

        let mut join_types = Vec::new();
        let mut join_columns = Vec::new();
        let mut join_generators = Vec::new();

        join_types.push(lhs_scheme.get_types()[lhs_column_index].clone());
        join_columns.push(lhs_scheme.get_columns()[lhs_column_index].clone());
        join_generators.push(lhs_scheme.get_validators()[lhs_column_index].clone());

        for i in lhs_range.clone() {
            if i != lhs_column_index {
                join_types.push("StringValue".to_owned());
                join_columns.push(lhs_scheme.get_columns()[i].clone());
                join_generators.push(self.supported_types.get("StringValue").unwrap().clone());
            }
        }

        for i in rhs_range.clone() {
            if i != rhs_column_index {
                join_types.push("StringValue".to_owned());
                join_columns.push(rhs_scheme.get_columns()[i].clone());
                join_generators.push(self.supported_types.get("StringValue").unwrap().clone());
            }
        }

        let join_scheme = Scheme::new(join_types, join_columns, join_generators);

        let join_table = match Table::builder()
        .with_name("join_table".to_string())
        .with_scheme(join_scheme)
        .build() {
            Ok(table) => table,
            Err(err) => return Err(err)
        };
        
        let mut lhs_core_column_values: Vec<String> = Vec::new();
        for i in 0..lhs.get_rows().len() {
            let cell_value = lhs.get_rows().get(i).unwrap().get_values().get(lhs_column_index).unwrap().clone();

            lhs_core_column_values.push(
                match cell_value.get_value() {
                    core::types::ValueType::Int(int) => {
                        int.get_value().to_string()
                    },
                    core::types::ValueType::Str(str) => {
                        str.get_value().to_owned()
                    },
                    core::types::ValueType::Real(real) => {
                        real.get_value().to_string()
                    },
                    core::types::ValueType::Pic(_picture) => {
                        "picture".to_owned()
                    },
                    core::types::ValueType::Char(char) => {
                        char.get_value().to_string()
                    },
                    core::types::ValueType::Date(date) => {
                        date.get_value().to_string()
                    },
                    core::types::ValueType::Email(email) => {
                        email.get_value().to_string()
                    }
                }
            )
        }
        let mut core_lhs_column_values_copy = lhs_core_column_values.clone();
        let lhs_len = core_lhs_column_values_copy.len();
        core_lhs_column_values_copy.sort();
        core_lhs_column_values_copy.dedup();
        if lhs_len != core_lhs_column_values_copy.len() {
            return Err("The table column is not consist from unique values".to_owned());
        }

        let mut rhs_core_column_values: Vec<String> = Vec::new();
        for i in 0..rhs.get_rows().len() {
            let cell_value = rhs.get_rows().get(i).unwrap().get_values().get(rhs_column_index).unwrap().clone();
            
            rhs_core_column_values.push(
                match cell_value.get_value() {
                    core::types::ValueType::Int(int) => {
                        int.get_value().to_string()
                    },
                    core::types::ValueType::Str(str) => {
                        str.get_value().to_owned()
                    },
                    core::types::ValueType::Real(real) => {
                        real.get_value().to_string()
                    },
                    core::types::ValueType::Pic(_picture) => {
                        "picture".to_owned()
                    },
                    core::types::ValueType::Char(char) => {
                        char.get_value().to_string()
                    },
                    core::types::ValueType::Date(date) => {
                        date.get_value().to_string()
                    },
                    core::types::ValueType::Email(email) => {
                        email.get_value().to_string()
                    }
                }
            )
        }
        let mut core_rhs_column_values_copy = rhs_core_column_values.clone();
        let rhs_len = core_rhs_column_values_copy.len();
        core_rhs_column_values_copy.sort();
        core_rhs_column_values_copy.dedup();
        if rhs_len != core_rhs_column_values_copy.len() {
            return Err("The table column is not consist from unique values".to_owned());
        }

        let mut core_column_values: Vec<String> = Vec::new();
        for value in lhs_core_column_values {
            core_column_values.push(value);
        }
        for value in rhs_core_column_values {
            core_column_values.push(value);
        }
        core_column_values.sort();
        core_column_values.dedup();

        for core_column_value in core_column_values {
            let mut row_values: Vec<String> = Vec::new();
            row_values.push(core_column_value);

            let lhs_row_index: Option<usize> = None;
            let rhs_row_index: Option<usize> = None;

            match lhs_row_index {
                Some(index) => {
                    let lhs_row = lhs.get_rows().get(index).unwrap().clone();
                    for lhs_cell in lhs_row.get_values() {
                        row_values.push(
                            match lhs_cell.get_value() {
                                core::types::ValueType::Int(int) => {
                                    int.get_value().to_string()
                                },
                                core::types::ValueType::Str(str) => {
                                    str.get_value().to_owned()
                                },
                                core::types::ValueType::Real(real) => {
                                    real.get_value().to_string()
                                },
                                core::types::ValueType::Pic(_picture) => {
                                    "picture".to_owned()
                                },
                                core::types::ValueType::Char(char) => {
                                    char.get_value().to_string()
                                },
                                core::types::ValueType::Date(date) => {
                                    date.get_value().to_string()
                                },
                                core::types::ValueType::Email(email) => {
                                    email.get_value().to_string()
                                }
                            }
                        )
                    }
                }
                None => {
                    for _ in lhs_range.clone() {
                        row_values.push("##NONE##".to_owned())
                    }
                },
            }
            match rhs_row_index {
                Some(index) => {
                    let rhs_row = rhs.get_rows().get(index).unwrap().clone();
                    for rhs_cell in rhs_row.get_values() {
                        row_values.push(
                            match rhs_cell.get_value() {
                                core::types::ValueType::Int(int) => {
                                    int.get_value().to_string()
                                },
                                core::types::ValueType::Str(str) => {
                                    str.get_value().to_owned()
                                },
                                core::types::ValueType::Real(real) => {
                                    real.get_value().to_string()
                                },
                                core::types::ValueType::Pic(_picture) => {
                                    "picture".to_owned()
                                },
                                core::types::ValueType::Char(char) => {
                                    char.get_value().to_string()
                                },
                                core::types::ValueType::Date(date) => {
                                    date.get_value().to_string()
                                },
                                core::types::ValueType::Email(email) => {
                                    email.get_value().to_string()
                                }
                            }
                        )
                    }
                }
                None => {
                    for _ in rhs_range.clone() {
                        row_values.push("##NONE##".to_owned())
                    }
                },
            }

            Self::add_row_to_the_table(&join_table, row_values);
        }
        
        Ok(join_table)
    }

    pub fn rename(&self, table_name: &str, new_columns_names: Vec<String>) -> Result<(), String> {
        if self.database.borrow().is_none() {
            return Err("There is no active databases in db-manager manager".to_string());
        }
        let db = self.database.borrow();
        let db_unwrapped = db.as_ref().unwrap();
        let res = match db_unwrapped.get_tables_mut().get_mut(table_name) {
            Some(table) => {
                let scheme = table.get_scheme_mut();
                if scheme.get_columns().len() != new_columns_names.len() {
                    return Err("wrong number of tables".to_string());
                }
                scheme.set_columns(new_columns_names);
                Ok(())
            },
            None => Err(format!("There is no table with name {}", table_name))
        };
        res
    }

    pub fn get_database_name(&self) -> String{
        self.database.borrow().as_ref().unwrap().get_name().to_owned()
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        let _ = self.close_db(true);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creating_db_manager() {
        // DatabaseManager::new();
    }
}