use crate::item::{Item, ItemStruct};
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct VectorItem {
    #[pyo3(get)]
    prefix: String,
    #[pyo3(get)]
    value: Vec<ItemStruct>,
}


impl VectorItem {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            value: vec![],
        }
    }

    pub fn push(&mut self, item: ItemStruct) {
        self.value.push(item);
    }
}
