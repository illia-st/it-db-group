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
        let database = match fs::read(format!("{}/{}", location, "tables")) {
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
            let mut file = match File::open(db_dto.location.as_str()) {
                Ok(file) => file,
                Err(err) => return Err(format!("couldn't open the file to save {}: {}", db_dto.name, err))
            };
            match file.write_all(db_dto.encode().as_slice()) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("couldn't write to the file to save {}: {}", db_dto.name, err)),
            }
        } else {
            Ok(())
        };
        res
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
    pub fn db_is_opened(&self) -> bool {
        self.database.borrow().is_some()
    }
    pub fn get_table_list(&self) -> Vec<String> {
        self.database.borrow().as_ref().unwrap().get_tables().keys().cloned().collect::<Vec<String>>()
    }

    pub fn join(&self, lhs_table_name: &str, rhs_table_name: &str, column: &str) -> Result<Table, String> {
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
        let mut lhs_rows = lhs.rows.borrow_mut();
        let mut rhs_rows = rhs.rows.borrow_mut();

        let mut lhs_columns = lhs.scheme.get_columns();
        let mut rhs_columns = rhs.scheme.get_columns();

        // 1) check if such column exist in columns, check it's type
        // meanwhile need to save columns with the same names and check the type of it
        // TODO: think of type checker
        // 2) if everything is fine - go further
        let mut scheme = Scheme::<dyn CellValue>::new(
            lhs.scheme.get_types(),
            lhs.scheme.get_columns(),
            lhs.scheme.get_validators().to_owned(),
        );
        let mut column_here = false;
        let mut column_index_l = 0;
        let mut column_index_r = 0;
        for (rhs_index, rhs_column) in rhs_columns.iter_mut().enumerate() {
            for (lhs_index, lhs_column) in lhs_columns.iter_mut().enumerate() {
                if rhs_column == lhs_column
                    && rhs.scheme.types[rhs_index] == scheme.types[lhs_index]
                    && rhs_column == column
                {
                    column_here = true;
                    column_index_l = lhs_index;
                    column_index_r = rhs_index;
                } else if rhs_column == lhs_column
                    && rhs.scheme.types[rhs_index] == scheme.types[lhs_index]
                {
                    rhs_column.insert_str(rhs_column.len(), "_rhs");
                    lhs_column.insert_str(lhs_column.len(), "_lhs");
                }
                if rhs_column != lhs_column {
                    scheme.columns.push(rhs_column.to_string());
                    scheme.types.push(rhs.scheme.get_types()[rhs_index].to_string());
                    scheme.value_generators.push(rhs.scheme.get_validators()[rhs_index].to_owned());
                }
            }
        }
        if !column_here {
            return Err(format!("Column wasn't found or found with different types: {}", column));
        }
        scheme.columns = lhs_columns;
        let mut ans_rows = lhs_rows.clone();
        for rhs_row in rhs_rows.iter_mut() {
            for (lhs_index, lhs_row) in lhs_rows.iter_mut().enumerate() {
                if rhs_row.get_values()[column_index_r].get_value() == lhs_row.get_values()[column_index_l].get_value() {
                    // have found same values
                    rhs_row.values.iter_mut().enumerate().for_each(|(index, value)| {
                        if index == column_index_r {
                            return;
                        }
                        // ans_rows[lhs_index].push_value(value.clone());
                    });
                } else {
                    let mut helper = rhs_row.get_values().to_vec();
                    ans_rows.push(todo!("push rhs row with adjusted indexes and column"));
                }
            }
            // iterate rhs_rows
        }

        let mut ans = lhs_rows.clone();
        for rhs_row in rhs_rows.iter() {
            let mut flag = false;
            for lhs_row in lhs_rows.iter() {
                let lhs_row_values = lhs_row.get_values();
                for (index, value) in lhs_row.get_values().iter().enumerate() {
                    if lhs_row_values[index].as_ref().get_value() == value.as_ref().get_value() {
                        flag = true;
                    }
                }
            }
            if !flag {
                ans.push(rhs_row.clone());
            }
        }
        let res = Table::builder()
            .with_scheme(lhs.get_scheme().clone())
            .with_name("Join".to_string())
            .build()
            .unwrap();
        res.set_rows(ans);
        Ok(res)
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