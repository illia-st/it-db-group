use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::row::Row;
use crate::scheme::Scheme;
use crate::types::CellValue;

#[derive(Clone)]
pub struct Table {
    name: String,
    #[allow(dead_code)]
    scheme: Scheme<dyn CellValue>,
    rows: RefCell<Vec<Rc<Row<dyn CellValue>>>>,
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
        if borrows_rows.len() as u64 >= index {
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