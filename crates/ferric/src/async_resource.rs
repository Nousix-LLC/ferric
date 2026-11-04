//! Async resource primitives — public API surface frozen by the walking skeleton.
//!
//! Full implementation is owned by SUBTASK_async_resource. The types and function
//! signatures here are the contract; bodies are `todo!()` placeholders until that
//! task completes.

use std::future::Future;

use crate::vdom::VNode;

/// The lifecycle state of an async-loaded value.
#[allow(dead_code)]
pub enum ResourceState<T> {
    /// The future is still running.
    Loading,
    /// The future completed successfully.
    Ready(T),
    /// The future completed with an error.
    Failed(String),
}

/// A reactive handle to an async-loaded value.
///
/// Create one with [`Resource::new`]; poll its current state with [`Resource::state`].
/// The state transitions from `Loading` to `Ready` or `Failed` exactly once.
#[allow(dead_code)]
pub struct Resource<T: 'static> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static> Resource<T> {
    /// Spawn `fut` and return a handle whose [`state`](Self::state) tracks the result.
    pub fn new(_fut: impl Future<Output = T> + 'static) -> Self {
        todo!("SUBTASK_async_resource: implement Resource::new")
    }

    /// Return the current lifecycle state of the resource.
    pub fn state(&self) -> ResourceState<T> {
        todo!("SUBTASK_async_resource: implement Resource::state")
    }
}

/// Render `fallback` while any child [`Resource`] is [`ResourceState::Loading`], otherwise
/// render `children`.
pub fn suspense(_fallback: impl Fn() -> VNode, _children: impl Fn() -> VNode) -> VNode {
    todo!("SUBTASK_async_resource: implement suspense")
}
