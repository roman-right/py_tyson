use crate::de::Desereilize;
use crate::item::{Item, ItemStruct};
use crate::primitive::PrimitiveItem;
use pyo3::prelude::*;

#[derive(Debug)]
#[pyclass]
pub struct Document {
    #[pyo3(get)]
    items: Vec<(PrimitiveItem, ItemStruct)>,
}

impl Desereilize for Document {
    fn new_document() -> Document {
        Document {
            items: vec![]
        }
    }

    fn add_to_document(&mut self, data: (PrimitiveItem, ItemStruct)) {
        self.items.push(data)
    }
}

// #[pymethods]
// impl Document {
//     #[get_items]
//     fn get_items(&self) {}
// }