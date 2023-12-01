use std::fmt::Debug;
use std::rc::Rc;
use super::types::CellValue;

#[derive(Debug)]
pub struct Row<T>
where
    T: CellValue + ?Sized + Debug
{
    pub values: Vec<Rc<T>>,
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

    pub fn push_value(&mut self, value: Rc<T>) {
        self.values.push(value);
    }
}

#[cfg(test)]
mod tests {

}