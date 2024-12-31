use loro::LoroList as LoroListInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroList(pub LoroListInner);

#[pymethods]
impl LoroList {}
