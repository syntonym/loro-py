use loro::{LoroMovableList as LoroMovableListInner, LoroValue};
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroMovableList(pub LoroMovableListInner);

#[pymethods]
impl LoroMovableList {
    #[new]
    pub fn new() -> Self {
        Self(LoroMovableListInner::new())
    }
}
