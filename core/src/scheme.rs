use std::rc::Rc;
use crate::types::CellValue;

pub struct Scheme<T>
where
    T: Default + CellValue + ?Sized,
{
    value_validators: Vec<fn(String) -> Rc<T>>,
}
impl<T> Scheme<T>
where
    T: Default + CellValue + ?Sized,
{
    fn new(value_validators: Vec<fn(String) -> Rc<T>>) -> Self {
        Self { value_validators }
    }
    pub fn builder() -> SchemeBuilder<T> {
        SchemeBuilder::<T>::default()
    }
    pub fn get_validators(&self) -> &[fn(String) -> Rc<T>] {
        self.value_validators.as_slice()
    }
}
#[derive(Default)]
pub struct SchemeBuilder<T>
where
    T: Default + CellValue + ?Sized,
{
    value_validators: Vec<fn(String) -> Rc<T>>
}

impl<T> SchemeBuilder<T>
where
    T: Default + CellValue + ?Sized,
{
    pub fn with_type(mut self, validator: fn(String) -> Rc<T>) -> Self {
        self.value_validators.push(validator);
        self
    }
    pub fn build(self) -> Scheme<T> {
        Scheme::<T>::new(self.value_validators)
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}