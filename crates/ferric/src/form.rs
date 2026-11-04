//! Two-way form binding primitives — public API surface frozen by the walking skeleton.
//!
//! Full implementation is owned by SUBTASK_form_binding. The types and method
//! signatures here are the contract; bodies are `todo!()` placeholders until that
//! task completes.

use std::str::FromStr;

use crate::vdom::EventHandler;

/// Typed form state wrapper.
///
/// Holds the raw string value of a form field and lazily parses it to `T`.
/// Create one with [`FormField::new`]; wire it to the DOM with [`Bind`].
#[allow(dead_code)]
pub struct FormField<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: FromStr + Clone + 'static> FormField<T>
where
    T::Err: std::fmt::Display,
{
    /// Create a field with the given initial raw string.
    pub fn new(_initial: &str) -> Self {
        todo!("SUBTASK_form_binding: implement FormField::new")
    }

    /// Return the current raw string value.
    pub fn raw(&self) -> String {
        todo!("SUBTASK_form_binding: implement FormField::raw")
    }

    /// Attempt to parse the raw value as `T`. Returns `Err` with the display-formatted parse error.
    pub fn parsed(&self) -> Result<T, String> {
        todo!("SUBTASK_form_binding: implement FormField::parsed")
    }

    /// Update the raw string value (triggers reactive updates).
    pub fn set_raw(&self, _raw: String) {
        todo!("SUBTASK_form_binding: implement FormField::set_raw")
    }
}

/// Two-way binding descriptor for a [`FormField`].
///
/// Produces the `value` attribute and an `input` event handler that together keep
/// the DOM input element and the [`FormField`] in sync.
#[allow(dead_code)]
pub struct Bind<T: FromStr + Clone + 'static>
where
    T::Err: std::fmt::Display,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T: FromStr + Clone + 'static> Bind<T>
where
    T::Err: std::fmt::Display,
{
    /// Create a [`Bind`] wrapping the given [`FormField`].
    pub fn new(_field: &FormField<T>) -> Self {
        todo!("SUBTASK_form_binding: implement Bind::new")
    }

    /// Return the `("value", raw_string)` attribute pair for the input element.
    pub fn value_attr(&self) -> (&'static str, String) {
        todo!("SUBTASK_form_binding: implement Bind::value_attr")
    }

    /// Return an [`EventHandler`] that calls [`FormField::set_raw`] with the new input value.
    pub fn on_input_handler(&self) -> EventHandler {
        todo!("SUBTASK_form_binding: implement Bind::on_input_handler")
    }
}
