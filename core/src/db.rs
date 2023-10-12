use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

use crate::table::Table;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub location: String,
    pub tables: RefCell<HashMap<String, Table>>,
}

impl Database {
    pub fn new(name: String, location: String) -> Self {
        Self {
            name,
            location,
            tables: RefCell::new(HashMap::default()),
        }
    }
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }

    pub fn get_tables(&self) -> Ref<HashMap<String, Table>> {
        self.tables.borrow()
    }
    pub fn get_tables_mut(&self) -> RefMut<HashMap<String, Table>> {
        self.tables.borrow_mut()
    }

    pub fn set_tables(&self, tables: HashMap<String, Table>) {
        *self.tables.borrow_mut() = tables;
    }
}
#[derive(Default)]
pub struct DatabaseBuilder {
    name: Option<String>,
    location: Option<String>,
}

impl DatabaseBuilder {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }
    pub fn build(self) -> Result<Database, String> {
        let name = match self.name {
            Some(name) => name,
            None => return Err("name wasn't specified while constructing the database".to_string())
        };
        let location = match self.location {
            Some(location) => location,
            None => return Err("location wasn't specified while constructing the database".to_string())
        };
        Ok(
            Database::new(name, location)
        )
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}