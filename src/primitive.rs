use pyo3::prelude::*;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
#[pyclass]
pub struct PrimitiveItem {
    #[pyo3(get)]
    prefix: String,
    #[pyo3(get)]
    value: String,
}

impl PrimitiveItem {
    pub fn new(prefix: String, value: String) -> Self {
        Self {
            prefix,
            value,
        }
    }
}