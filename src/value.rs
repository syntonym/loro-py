use std::fmt::Display;
use std::sync::Arc;

use fxhash::FxHashMap;
use loro::{Counter, PeerID};
use pyo3::prelude::*;

use crate::container::Container;

pub fn register_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ID>()?;
    m.add_class::<ContainerType>()?;
    m.add_class::<ContainerID>()?;
    m.add_class::<LoroValue>()?;
    m.add_class::<LoroBinaryValue>()?;
    m.add_class::<LoroStringValue>()?;
    m.add_class::<LoroListValue>()?;
    m.add_class::<LoroMapValue>()?;
    m.add_class::<Ordering>()?;
    m.add_class::<TreeID>()?;
    m.add_class::<TreeParentId>()?;
    Ok(())
}

#[pyclass(eq, str, get_all, set_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID {
    pub peer: u64,
    pub counter: i32,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pymethods]
impl ID {
    #[new]
    pub fn new(peer: u64, counter: i32) -> Self {
        Self { peer, counter }
    }
}

#[pyclass(eq, hash, str)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerType {
    Text {},
    Map {},
    List {},
    MovableList {},
    Tree {},
    Counter {},
    Unknown { kind: u8 },
}

impl Display for ContainerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pyclass(eq, str, hash)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContainerID {
    Root {
        name: String,
        container_type: ContainerType,
    },
    Normal {
        peer: u64,
        counter: i32,
        container_type: ContainerType,
    },
}

impl Display for ContainerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pyclass(eq, str)]
#[derive(Debug, Clone, PartialEq)]
pub enum LoroValue {
    Null {},
    Bool(bool),
    Double(f64),
    I64(i64),
    Binary(LoroBinaryValue),
    String(LoroStringValue),
    List(LoroListValue),
    Map(LoroMapValue),
    Container(ContainerID),
}

impl Display for LoroValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pyclass]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LoroBinaryValue(pub Arc<Vec<u8>>);
#[pyclass]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LoroStringValue(pub Arc<String>);
#[pyclass]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LoroListValue(pub Arc<Vec<LoroValue>>);

#[pyclass]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LoroMapValue(pub Arc<FxHashMap<String, LoroValue>>);

#[pyclass(eq, str, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::cmp::Ordering> for Ordering {
    fn from(value: std::cmp::Ordering) -> Self {
        match value {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Greater,
        }
    }
}

#[pyclass(eq, str, get_all, set_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TreeID {
    pub peer: PeerID,
    pub counter: Counter,
}

impl Display for TreeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pyclass(eq, str)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeParentId {
    Node { id: TreeID },
    Root {},
    Deleted {},
    Unexist {},
}

impl Display for TreeParentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, FromPyObject, IntoPyObject)]
pub enum ValueOrContainer {
    Value(LoroValue),
    Container(Container),
}

impl Display for ValueOrContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
