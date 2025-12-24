use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Decides how traces will be recorded.
#[derive(Default)]
pub enum TraceStyle {
    /// Traces will be recorded as a group of threads.
    /// In this style, spans should be entered and exited on the same thread.
    #[default]
    Threaded,
    /// Traces will recorded as a group of asynchronous operations.
    Async,
}
