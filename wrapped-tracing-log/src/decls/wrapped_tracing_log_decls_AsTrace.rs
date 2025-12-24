use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait implemented for `log` types that can be converted to a `tracing`
/// equivalent.
pub trait AsTrace: crate::sealed::Sealed {
    /// The `tracing` type that this type can be converted into.
    type Trace;
    /// Returns the `tracing` equivalent of `self`.
    fn as_trace(&self) -> Self::Trace;
}
