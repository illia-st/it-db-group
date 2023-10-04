use std::cell::RefCell;
use crate::row::Row;
use crate::scheme::Scheme;
use crate::types::CellValue;

pub struct Table {
    name: String,
    #[allow(dead_code)]
    scheme: Scheme<dyn CellValue>,
    rows: RefCell<Vec<Row<dyn CellValue>>>,
}

impl Table
{
    fn new(name: String, scheme: Scheme<dyn CellValue>) -> Self {
        Self {
            name,
            scheme,
            rows: RefCell::new(Vec::default()),
        }
    }
    pub fn add_row(&self, _new_row: Row<dyn CellValue>) {
        todo!("add scheme validation");
        // self.rows.borrow_mut().push(new_row);
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