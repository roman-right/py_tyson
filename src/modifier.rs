use crate::item::{Item, ItemStruct};
use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass]
pub struct ModifierItem {
    #[pyo3(get)]
    prefix: String,
    value: Box<ItemStruct>,
}

impl ModifierItem {
    pub fn new(prefix: String, value: ItemStruct) -> Self {
        Self {
            prefix,
            value: Box::new(value),
        }
    }
}

#[pymethods]
impl ModifierItem {
    pub fn get_value(&self) -> ItemStruct {
        self.value.as_ref().clone()
    }
}