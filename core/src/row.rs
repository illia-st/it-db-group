use std::fmt::Debug;
use std::rc::Rc;
use crate::types::CellValue;

#[derive(Debug)]
pub struct Row<T>
where
    T: CellValue + ?Sized + Debug
{
    values: Vec<Rc<T>>,
}

impl<T> Row<T>
where
    T: CellValue + ?Sized + Debug
{
    pub fn new(values: Vec<Rc<T>>) -> Self {
        Self { values }
    }
    pub fn get_values(&self) -> &[Rc<T>] {
        self.values.as_slice()
    }
}

#[cfg(test)]
mod tests {

}