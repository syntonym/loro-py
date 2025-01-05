use loro::LoroUnknown as LoroUnknownInner;
use pyo3::prelude::*;

use crate::value::ContainerID;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub struct LoroUnknown(pub LoroUnknownInner);

#[pymethods]
impl LoroUnknown {
    /// Get the container id.
    #[getter]
    pub fn id(&self) -> ContainerID {
        self.0.id().into()
    }
}
