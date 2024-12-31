use pyo3::{exceptions::PyValueError, PyErr};

pub enum PyLoroError {
    LoroError(loro::LoroError),
    PyError(PyErr),
}

pub type PyLoroResult<T> = Result<T, PyLoroError>;

impl From<loro::LoroError> for PyLoroError {
    fn from(other: loro::LoroError) -> Self {
        Self::LoroError(other)
    }
}

impl From<PyLoroError> for PyErr {
    fn from(value: PyLoroError) -> Self {
        match value {
            PyLoroError::LoroError(e) => PyValueError::new_err(e.to_string()),
            PyLoroError::PyError(e) => e,
        }
    }
}

impl From<PyErr> for PyLoroError {
    fn from(value: PyErr) -> Self {
        Self::PyError(value)
    }
}
