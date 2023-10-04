use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use std::rc::Rc;
use crate::db::Database;
use core::types::CellValue;
use toml::Table;
use core::scheme::Scheme;
use core::row::Row;

// Can operate with one db at the time
#[derive(Debug)]
struct DatabaseConfig {
    #[allow(dead_code)]
    supported_types: Vec<String>,
}

#[derive(Default)]
pub struct DatabaseManager {

    #[allow(dead_code, clippy::type_complexity)]
    supported_types: HashMap<String, fn(String) -> Rc<dyn CellValue>>,
    database: RefCell<Option<Database>>,
}

impl DatabaseManager {
    // creating a database manager
    pub fn new() -> Self {
        // read db manager config
        let file_path = format!("{}/.config/config.toml", env!("CARGO_MANIFEST_DIR"));

        // Open the file for reading
        let mut file = File::open(file_path).expect("Failed to open file");

        // Read the file's contents into a String
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)
            .expect("Failed to read file");

        // Parse the TOML string into a toml::Value object
        let parsed_toml = toml_str.parse::<Table>().unwrap();
        // let parsed_toml: Value = toml::from_str(&toml_str).expect("Failed to parse TOML");
        parsed_toml["supported_types"]
            .as_array()
            .unwrap()
            .iter()
            .for_each(|value| {
                if let toml::Value::String(_supported_type) = value {

                }
            })
        ;

        todo!()
    }
    pub fn create_db(&self, name: &str, location: &str) -> Result<(), String> {
        let _ = self.close_db();
        // check fi such a dir is existing
        if let Ok(metadata) = fs::metadata(location) {
            if !metadata.is_dir() {
                return Err("provided path points to the file or symlink".to_string());
            }
        }
        // create a dir
        match fs::create_dir(format!("{}/{}", location, name)) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a directory: {err}"))
        }
        // create a dir for tables
        match File::create(format!("{}/{}/{}", location, name, "tables")) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a file: {err}"))
        }
        // build db using Database::builder()
        let database = Database::builder()
            .with_location(location)
            .with_name(name)
            .build()
            .unwrap();
        *self.database.borrow_mut().deref_mut() = Some(database);
        Ok(())
    }

    pub fn read_db_from_directory(&self, location: &str) -> Result<(), String> {
        // need to close the previous one
        let _ = self.close_db();
        // check if provided location is a dir
        match fs::metadata(location) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err("provided path points to the file or symlink".to_string());
                }
            },
            Err(err) => return Err(format!("couldn't open a directory {}: {}", location, err))
        };
        // read file location/table using amazon ion
        let tables = match fs::read_dir(format!("{}/{}", location, "tables")) {
            Ok(tables) => tables,
            Err(err) => {
                let err_string = format!("The error is occurred while trying to read tables: {}", err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            }
        };
        tables.for_each(|entry| {
            let unwrapped_entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    let err_string = format!("The error is occurred while trying to read tables: {}", err);
                    log::error!("{}", err_string.as_str());
                    return;
                },
            };
            match fs::read(unwrapped_entry.path()) {
                Ok(binary_data) => self.add_table(binary_data),
                Err(err) => log::error!("The error is occurred while trying to read tables: {}", err),
            };
        });
        Ok(())
    }
    fn add_table(&self, _raw_table_data: Vec<u8>) {
        todo!("add ion data structures here")
    }
    pub fn create_table(&self, table_name: String, _scheme: Scheme<dyn CellValue>) -> Result<(), String> {
        // 1) check if the table already exists
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        match File::create(format!("{}/{}/{}", self.database.borrow().as_ref().unwrap().get_location(), "tables", table_name)) {
            Ok(_table) => {
                // add ion data type for table adding
            },
            Err(err) => {
                let err_string = format!("Couldn't create a new table {}: {}", table_name, err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            },
        }
        todo!("unfinished with scheme");
        // Ok(())
    }
    pub fn delete_table(&self, table_name: &str) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        match fs::remove_file(format!("{}/{}/{}", self.database.borrow().as_ref().unwrap().get_location(), "tables", table_name)) {
            Ok(()) => {
                log::debug!("table {} has been removed", table_name);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete table {}: {}", table_name, err);
                log::error!("{}", err_string.as_str());
                Err(err_string)
            },
        }
    }
    pub fn add_row(&self, table_name: &str, raw_values: &str) -> Result<(), String>{
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }

        let db = self.database.borrow();
        let db_unwrapped = db.as_ref().unwrap();
        let res = match db_unwrapped.get_tables_mut().get_mut(table_name) {
            Some(table) => {
                let scheme = table.get_scheme();
                // TODO: add ion schema instead of using &str for raw values
                // FIXME: bad splitter, need to use ion schema
                let split_values: Vec<String> = raw_values
                    .split(':')
                    .map(|value| value.trim().to_string())
                    .collect();
                let mut row_values = Vec::with_capacity(split_values.len());
                for (index, generator) in scheme.get_validators().iter().enumerate() {
                    match generator(split_values[index].clone()) {
                        Ok(value) => row_values.push(value),
                        Err(err) => {
                            log::error!("{err}");
                            return Err(err);
                        }
                    }
                }
                let new_row = Row::new(row_values);
                log::debug!("Added row into table {} with values {:?}", table_name, new_row);
                table.add_row(new_row);
                Ok(())
            },
            None => {
                let err_string = format!("There is no table {} in {}", table_name, db_unwrapped.get_name());
                log::error!("{}", err_string.as_str());
                Err(err_string)
            }
        };
        res
    }
    pub fn delete_row(&self, table_name: &str, index: u64) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
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
    pub fn close_db(&self) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        let mut db = self.database.borrow_mut();
        db.as_ref().unwrap().get_tables().iter().for_each(|_table| {
           // dump all the tables into location/tables/table_name
        });
        *db.deref_mut() = None;
        Ok(())
    }
    pub fn delete_db(&self, location: &str) -> Result<(), String> {
        // TODO: it will be nice to check if the provided location actually is a db
        match fs::remove_dir_all(location) {
            Ok(()) => {
                log::debug!("db in {} has been removed", location);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete db in {}: {}", location, err);
                log::error!("{}", err_string.as_str());
                Err(err_string)
            },
        }
    }
    pub fn get_db(&self) {
        todo!()
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        let _ = self.close_db();
    }
}

#[cfg(test)]
mod tests {
    use crate::db_manager::DatabaseManager;

    #[test]
    fn test_creating_db_manager() {
        // DatabaseManager::new();
    }
}