use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use core::db::Database;
use core::types::CellValue;
use core::scheme::Scheme;
use core::row::Row;
use core::types::SUPPORTED_TYPES;
use core::table::Table;

// Can operate with one db-manager at the time
#[derive(Default)]
pub struct DatabaseManager {
    #[allow(clippy::type_complexity)]
    supported_types: HashMap<String, Arc<fn(String) -> Result<Rc<dyn CellValue>, String>>>,
    database: RefCell<Option<Database>>,
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

    pub fn read_db_from_directory(&self, location: &str) -> Result<(), String> {
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
        let _database = match fs::read(format!("{}/{}", location, "tables")) {
            Ok(database) => database,
            Err(err) => {
                let err_string = format!("The error is occurred while trying to read tables: {}", err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            }
        };
        // TODO: use ion dto structures to convert database Vec<u8> into Database structure
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
        for (data_type, column_name) in data_types.iter().zip(columns) {
            match self.supported_types.get(*data_type) {
                Some(value_generator) => value_generators.push(value_generator.clone()),
                None => return Err(format!("No such supported data type: {}", data_type))
            }
            new_columns.push(column_name.to_string());
        }
        let scheme = Scheme::new(new_columns, value_generators);
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
    pub fn delete_row(&self, table_name: &str, index: u64) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db-manager manager";
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
    pub fn close_db(&self, save: bool) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db-manager manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        let mut db = self.database.borrow_mut();
        if save {
            db.as_ref().unwrap().get_tables().iter().for_each(|_table| {
                todo!("dump the db into the file in binary format")
            });
        }
        *db.deref_mut() = None;
        Ok(())
    }
    pub fn delete_db(&self, location: &str) -> Result<(), String> {
        // TODO: it will be nice to check if the provided location actually is a db but who cares?
        match fs::remove_dir_all(location) {
            Ok(()) => {
                log::debug!("db-manager in {} has been removed", location);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete db-manager in {}: {}", location, err);
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