use pyo3::prelude::*;

use crate::de::Desereilize;
use crate::item::{Item, ItemStruct};
use crate::primitive::PrimitiveItem;

#[derive(Debug)]
#[pyclass]
pub struct Journal {
    #[pyo3(get)]
    data: Vec<(PrimitiveItem, ItemStruct)>,
}

impl Desereilize for Journal {
    fn new() -> Journal {
        Journal {
            data: vec![]
        }
    }

    fn push(&mut self, data: (PrimitiveItem, ItemStruct)) {
        self.data.push(data)
    }
}