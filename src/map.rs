use std::collections::HashMap;

use crate::item::{Item, ItemStruct};
use crate::primitive::PrimitiveItem;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct MapItem {
    #[pyo3(get)]
    prefix: String,
    #[pyo3(get)]
    value: Vec<(PrimitiveItem, ItemStruct)>,
}


impl MapItem {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            value: vec![],
        }
    }

    pub fn insert(&mut self, k: PrimitiveItem, v: ItemStruct) {
        self.value.push((k, v.clone()));
    }
}