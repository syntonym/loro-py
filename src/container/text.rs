use loro::LoroText as LoroTextInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroText(pub LoroTextInner);

#[pymethods]
impl LoroText {
    #[new]
    pub fn new() -> Self {
        Self(LoroTextInner::new())
    }
}
