use loro::LoroUnknown as LoroUnknownInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroUnknown(pub LoroUnknownInner);

#[pymethods]
impl LoroUnknown {}
