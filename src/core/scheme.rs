#![allow(clippy::type_complexity)]
use std::rc::Rc;
use std::sync::Arc;
use super::types::CellValue;

#[derive(Debug)]
pub struct Scheme<T>
where
    T: CellValue + ?Sized,
{
    pub types: Vec<String>,
    pub value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>,
    // TODO: add columns name
    pub columns: Vec<String>,
}
impl<T> Clone for Scheme<T>
where
    T: CellValue + ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            types: self.types.clone(),
            value_generators: self.value_generators.clone(),
            columns: self.columns.clone(),
        }
    }
}

impl<T> Scheme<T>
where
    T: CellValue + ?Sized,
{
    pub fn new(types: Vec<String>, columns: Vec<String>, value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>) -> Self {
        Self {
            types,
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
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }

    pub fn get_types(&self) -> Vec<String> {
        self.types.clone()
    }
    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }
}
#[derive(Default)]
pub struct SchemeBuilder<T>
where
    T: CellValue + ?Sized,
{
    types: Vec<String>,
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
            types: Vec::default(),
        }
    }

    pub fn with_column(mut self, ty: String, column: String, validator: Arc<fn(String) -> Result<Rc<T>, String>>) -> Self {
        self.value_validators.push(validator);
        self.columns.push(column);
        self.types.push(ty);
        self
    }

    pub fn build(self) -> Scheme<T> {
        Scheme::<T>::new(self.types, self.columns, self.value_validators)
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}