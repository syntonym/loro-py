use pyo3::prelude::*;

use crate::{
    container::Cursor,
    doc::{AbsolutePosition, CounterSpan, LoroDoc},
    err::PyLoroResult,
    event::DiffEvent,
    value::LoroValue,
};

pub fn register_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<UndoManager>()?;
    m.add_class::<UndoOrRedo>()?;
    Ok(())
}

#[pyclass]
pub struct UndoManager(loro::UndoManager);

#[pymethods]
impl UndoManager {
    /// Create a new UndoManager.
    #[new]
    pub fn new(doc: &LoroDoc) -> Self {
        Self(loro::UndoManager::new(&doc.doc))
    }

    /// Undo the last change made by the peer.
    pub fn undo(&mut self) -> PyLoroResult<bool> {
        Ok(self.0.undo()?)
    }

    /// Redo the last change made by the peer.
    pub fn redo(&mut self) -> PyLoroResult<bool> {
        Ok(self.0.redo()?)
    }

    /// How many times the undo manager can undo.
    pub fn undo_count(&self) -> usize {
        self.0.undo_count()
    }

    /// How many times the undo manager can redo.
    pub fn redo_count(&self) -> usize {
        self.0.redo_count()
    }

    /// Record a new checkpoint.
    pub fn record_new_checkpoint(&mut self) -> PyLoroResult<()> {
        Ok(self.0.record_new_checkpoint()?)
    }

    /// Whether the undo manager can undo.
    pub fn can_undo(&self) -> bool {
        self.0.can_undo()
    }

    /// Whether the undo manager can redo.
    pub fn can_redo(&self) -> bool {
        self.0.can_redo()
    }

    /// If a local event's origin matches the given prefix, it will not be recorded in the
    /// undo stack.
    pub fn add_exclude_origin_prefix(&mut self, prefix: &str) {
        self.0.add_exclude_origin_prefix(prefix)
    }

    /// Set the maximum number of undo steps. The default value is 100.
    pub fn set_max_undo_steps(&mut self, size: usize) {
        self.0.set_max_undo_steps(size)
    }

    /// Set the merge interval in ms. The default value is 0, which means no merge.
    pub fn set_merge_interval(&mut self, interval: i64) {
        self.0.set_merge_interval(interval)
    }

    /// Set the listener for push events.
    /// The listener will be called when a new undo/redo item is pushed into the stack.
    pub fn set_on_push(&mut self, on_push: Py<PyAny>) {
        self.0
            .set_on_push(Some(Box::new(move |undo_or_redo, span, event| {
                Python::attach(|py| {
                    let meta = on_push
                        .call1(
                            py,
                            (
                                UndoOrRedo::from(undo_or_redo),
                                CounterSpan::from(span),
                                event.map(DiffEvent::from),
                            ),
                        )
                        .unwrap()
                        .extract::<UndoItemMeta>(py)
                        .unwrap();
                    loro::undo::UndoItemMeta::from(meta)
                })
            })));
    }

    /// Set the listener for pop events.
    /// The listener will be called when an undo/redo item is popped from the stack.
    pub fn set_on_pop(&mut self, on_pop: Py<PyAny>) {
        self.0
            .set_on_pop(Some(Box::new(move |undo_or_redo, span, meta| {
                Python::attach(|py| {
                    on_pop
                        .call1(
                            py,
                            (
                                UndoOrRedo::from(undo_or_redo),
                                CounterSpan::from(span),
                                UndoItemMeta::from(meta),
                            ),
                        )
                        .unwrap();
                })
            })));
    }

    /// Clear the undo stack and the redo stack
    pub fn clear(&self) {
        self.0.clear();
    }

    /// Will start a new group of changes, all subsequent changes will be merged
    /// into a new item on the undo stack. If we receive remote changes, we determine
    /// whether or not they are conflicting. If the remote changes are conflicting
    /// we split the undo item and close the group. If there are no conflict
    /// in changed container ids we continue the group merge.
    pub fn group_start(&mut self) -> PyLoroResult<()> {
        Ok(self.0.group_start()?)
    }

    /// Ends the current group, calling UndoManager::undo() after this will
    /// undo all changes that occurred during the group.
    pub fn group_end(&mut self) {
        self.0.group_end()
    }

    /// Get the peer id of the undo manager
    pub fn peer(&self) -> u64 {
        self.0.peer()
    }

    /// Get the metadata of the top undo stack item, if any.
    pub fn top_undo_meta(&self) -> Option<UndoItemMeta> {
        self.0.top_undo_meta().map(UndoItemMeta::from)
    }

    /// Get the metadata of the top redo stack item, if any.
    pub fn top_redo_meta(&self) -> Option<UndoItemMeta> {
        self.0.top_redo_meta().map(UndoItemMeta::from)
    }

    /// Get the value associated with the top undo stack item, if any.
    pub fn top_undo_value(&self) -> Option<LoroValue> {
        self.0.top_undo_value().map(Into::into)
    }

    /// Get the value associated with the top redo stack item, if any.
    pub fn top_redo_value(&self) -> Option<LoroValue> {
        self.0.top_redo_value().map(Into::into)
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoOrRedo {
    Undo,
    Redo,
}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct UndoItemMeta {
    pub value: LoroValue,
    pub cursors: Vec<CursorWithPos>,
}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct CursorWithPos {
    pub cursor: Cursor,
    pub pos: AbsolutePosition,
}
