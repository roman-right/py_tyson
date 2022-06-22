pub mod de;
pub mod primitive;
pub mod errors;
mod map;
mod item;
mod vector;
mod document;
mod modifier;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pyo3::prelude::*;
use crate::de::Desereilize;
use crate::document::Document;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn deserialize(data: String) -> PyResult<Py<Document>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let doc = Document::deserialize(data)?;
    Py::new(py, doc)
}

/// A Python module implemented in Rust.
#[pymodule]
fn tyson(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Document>()?;
    m.add_function(wrap_pyfunction!(deserialize, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::de::Desereilize;
    use crate::document::Document;

    #[test]
    fn de_se() {
        let data = r#"
        l|123|: hash{|s|: s|100|; a:s|100|};
        l|124|: {c|1|: s|100|, b|2|:s|100|},
        l|123|: ll[d|1|, e|2|],
        l|123|: [d|1|, e|2|],
        "#;

        let doc = Document::deserialize(data.to_string()).unwrap();
        print!("{:?}", doc);
    }
}
