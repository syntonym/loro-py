use pyo3::prelude::*;

mod counter;
mod list;
mod map;
mod movable_list;
mod text;
mod tree;

pub use counter::LoroCounter;
pub use list::LoroList;
pub use map::LoroMap;
pub use movable_list::LoroMovableList;
pub use text::LoroText;
pub use tree::LoroTree;

#[pyclass(frozen)]
#[derive(Debug, Clone)]
pub enum Container {
    List(LoroList),
    Map(LoroMap),
    MovableList(LoroMovableList),
    Text(LoroText),
    Tree(LoroTree),
    Counter(LoroCounter),
}

pub fn register_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Container>()?;
    m.add_class::<LoroList>()?;
    m.add_class::<LoroMap>()?;
    m.add_class::<LoroText>()?;
    m.add_class::<LoroTree>()?;
    m.add_class::<LoroMovableList>()?;
    m.add_class::<LoroCounter>()?;
    Ok(())
}
