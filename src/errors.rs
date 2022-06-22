use pest::error::Error;
use pyo3::PyErr;
use pyo3::exceptions::PyValueError;

use crate::de::Rule;

#[derive(Debug)]
pub struct TySONError {
    msg: String,
}

impl TySONError {
    pub(crate) fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string()
        }
    }

    pub fn unexpected_parsing() -> Self {
       Self::new("Unexpected parsing error")
    }
}

impl From<Error<Rule>> for TySONError {
    fn from(error: Error<Rule>) -> Self {
        Self {
            msg: error.to_string()
        }
    }
}

impl From<TySONError> for PyErr{
    fn from(e: TySONError) -> Self {
        let msg = e.msg.clone();
        PyValueError::new_err(msg)
    }
}