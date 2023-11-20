use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::row::Row;
use crate::scheme::Scheme;
use crate::types::{CellValue};

#[derive(Clone, Debug)]
pub struct Table {
    pub name: String,
    #[allow(dead_code)]
    pub scheme: Scheme<dyn CellValue>,
    pub rows: RefCell<Vec<Rc<Row<dyn CellValue>>>>,
}

impl Table
{
    pub fn new(name: String, scheme: Scheme<dyn CellValue>) -> Self {
        Self {
            name,
            scheme,
            rows: RefCell::new(Vec::default()),
        }
    }
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }
    pub fn add_row(&self, new_row: Row<dyn CellValue>) {
        self.rows.borrow_mut().push(Rc::new(new_row));
    }
    pub fn pop(&self) {
        self.rows.borrow_mut().pop();
    }

    pub fn erase(&self, index: u64) -> Result<(), String> {
        let mut borrows_rows = self.rows.borrow_mut();
        if index > borrows_rows.len() as u64 {
            return Err(format!(
                "index is bigger that actual table size. Table - {}, size - {}, requested index - {}",
                self.name.as_str(),
                borrows_rows.len(),
                index
            ));
        }
        borrows_rows.remove(index as usize);
        Ok(())
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_scheme(&self) -> &Scheme<dyn CellValue> {
        &self.scheme
    }

    pub fn get_scheme_mut(&mut self) -> &mut Scheme<dyn CellValue> {
        &mut self.scheme
    }

    pub fn get_rows(&self) -> Ref<Vec<Rc<Row<dyn CellValue>>>> {
        self.rows.borrow()
    }
    pub fn get_columns(&self) -> Vec<String> {
        self.scheme.get_columns()
    }
    pub fn set_rows(&self, rows: Vec<Rc<Row<dyn CellValue>>>) {
        *self.rows.borrow_mut() = rows;
    }

    pub fn get_column_index(&self, column_name: &str) -> usize {
        self.get_columns().iter().position(|e| e == column_name).unwrap()
    }

    pub fn get_max_column_len(&self, column_name: &str) -> usize {
        let mut max_size = if column_name.len() < 5 {
            5
        } else {
            column_name.len()
        };
        
        let column_index = self.get_column_index(column_name);
        
        for row in self.get_rows().iter() {
            let local_size = match row.get_values().get(column_index).unwrap().get_value() {
                crate::types::ValueType::Int(int) => {
                    int.get_value().to_string().len()
                },
                crate::types::ValueType::Str(string) => {
                    string.get_value().len()
                },
                crate::types::ValueType::Real(real) => {
                    real.get_value().to_string().len()
                },
                crate::types::ValueType::Pic(_) => {
                    7
                },
                crate::types::ValueType::Char(_) => {
                    1
                },
                crate::types::ValueType::Date(date) => {
                    date.get_value().to_string().len()
                },
                crate::types::ValueType::Email(email) => {
                    email.get_value().to_string().len()
                },
            };
            if local_size > max_size {
                max_size = local_size;
            }
        }
        max_size
    }
}
#[derive(Default)]
pub struct TableBuilder {
    name: Option<String>,
    scheme: Option<Scheme<dyn CellValue>>,
}

impl TableBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_scheme(mut self, scheme: Scheme<dyn CellValue>) -> Self {
        self.scheme = Some(scheme);
        self
    }
    pub fn build(self) -> Result<Table, String> {
        let scheme = match self.scheme {
            Some(scheme) => scheme,
            None => return Err("scheme wasn't specified while constructing the table".to_string())
        };
        let name = match self.name {
            Some(name) => name,
            None => return Err("name wasn't specified while constructing the table".to_string())
        };
        Ok(
            Table::new(name, scheme)
        )
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}