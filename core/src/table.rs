use std::cell::RefCell;
use crate::row::Row;
use crate::types::CellValue;

#[derive(Default)]
pub struct Table<T>
where
    T: CellValue + Default + ?Sized
{
    name: String,
    location: String,
    rows: RefCell<Vec<Row<T>>>
}

impl<T> Table<T>
where
    T: CellValue + Default + ?Sized
{
    fn new(name: String, location: String) -> Self {
        Self {
            name,
            location,
            rows: RefCell::new(Vec::default()),
        }
    }
    pub fn add_row(&self, new_row: Row<T>) {
        self.rows.borrow_mut().push(new_row);
    }
    pub fn pop(&self) {
        self.rows.borrow_mut().pop();
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
}
#[derive(Default)]
pub struct TableBuilder<T>
where
    T: CellValue + Default + ?Sized
{
    name: Option<String>,
    location: Option<String>,
    marker: std::marker::PhantomData<T>
}

impl<T> TableBuilder<T>
where
    T: CellValue + Default + ?Sized
{
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
    pub fn build(self) -> Result<Table<T>, String> {
        let location = match self.location {
            Some(location) => location,
            None => return Err("location wasn't specified while constructing the table".to_string())
        };
        let name = match self.name {
            Some(name) => name,
            None => return Err("name wasn't specified while constructing the table".to_string())
        };
        Ok(
            Table::<T>::new(name, location)
        )
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}