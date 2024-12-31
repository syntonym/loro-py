use loro::LoroMap as LoroMapInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroMap(pub LoroMapInner);

#[pymethods]
impl LoroMap {
    #[new]
    pub fn new() -> Self {
        Self(LoroMapInner::new())
    }
}
