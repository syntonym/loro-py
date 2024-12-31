use loro::LoroMovableList as LoroMovableListInner;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone, Default)]
pub struct LoroMovableList(pub LoroMovableListInner);

#[pymethods]
impl LoroMovableList {
    #[new]
    pub fn new() -> Self {
        Self::default()
    }
}
