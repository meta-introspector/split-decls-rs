use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Either transmutation is allowed, we have an error, or we have an optional
/// Condition that must hold.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Answer<R, T> {
    Yes,
    No(Reason<T>),
    If(Condition<R, T>),
}
