use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type of binding to use when generating a pattern.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BindStyle {
    /// `x`
    Move,
    /// `mut x`
    MoveMut,
    /// `ref x`
    Ref,
    /// `ref mut x`
    RefMut,
}
