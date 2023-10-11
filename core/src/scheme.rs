#![allow(clippy::type_complexity)]
use std::rc::Rc;
use std::sync::Arc;
use crate::types::CellValue;

pub struct Scheme<T>
where
    T: CellValue + ?Sized,
{
    value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>,
    // TODO: add columns name
    columns: Vec<String>,
}
impl<T> Clone for Scheme<T>
where
    T: CellValue + ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            value_generators: self.value_generators.clone(),
            columns: self.columns.clone(),
        }
    }
}

impl<T> Scheme<T>
where
    T: CellValue + ?Sized,
{
    pub fn new(columns: Vec<String>, value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>) -> Self {
        Self {
            value_generators,
            columns,
        }
    }
    pub fn builder() -> SchemeBuilder<T> {
        SchemeBuilder::<T>::new()
    }
    pub fn get_validators(&self) -> &[Arc<fn(String) -> Result<Rc<T>, String>>] {
        self.value_generators.as_slice()
    }
}
#[derive(Default)]
pub struct SchemeBuilder<T>
where
    T: CellValue + ?Sized,
{
    value_validators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>,
    columns: Vec<String>,
}

impl<T> SchemeBuilder<T>
where
    T: CellValue + ?Sized,
{
    fn new() -> Self {
        Self {
            value_validators: Vec::default(),
            columns: Vec::default(),
        }
    }

    pub fn with_column(mut self, column: String, validator: Arc<fn(String) -> Result<Rc<T>, String>>) -> Self {
        self.value_validators.push(validator);
        self.columns.push(column);
        self
    }

    pub fn build(self) -> Scheme<T> {
        Scheme::<T>::new(self.columns, self.value_validators)
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}