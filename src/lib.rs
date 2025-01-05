#![doc = include_str!("../README.md")]
use pyo3::prelude::*;

mod awareness;
mod container;
mod convert;
mod doc;
mod err;
mod event;
mod undo;
mod value;
mod version;

/// Python bindings for Loro
#[pymodule]
fn loro(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    doc::register_class(m)?;
    container::register_class(m)?;
    event::register_class(m)?;
    value::register_class(m)?;
    version::register_class(m)?;
    undo::register_class(m)?;
    awareness::register_class(m)?;
    Ok(())
}
