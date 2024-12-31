use pyo3::{exceptions::PyValueError, PyErr};

pub struct PyLoroError(loro::LoroError);

pub type PyLoroResult<T> = Result<T, PyLoroError>;

impl From<loro::LoroError> for PyLoroError {
    fn from(other: loro::LoroError) -> Self {
        Self(other)
    }
}

impl From<PyLoroError> for PyErr {
    fn from(value: PyLoroError) -> Self {
        PyValueError::new_err(value.0.to_string())
    }
}
