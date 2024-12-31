use loro::LoroCounter as LoroCounterInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroCounter(pub LoroCounterInner);

#[pymethods]
impl LoroCounter {}
