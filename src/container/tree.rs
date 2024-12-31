use loro::LoroTree as LoroTreeInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroTree(pub LoroTreeInner);

#[pymethods]
impl LoroTree {
    #[new]
    pub fn new() -> Self {
        Self(LoroTreeInner::new())
    }
}
