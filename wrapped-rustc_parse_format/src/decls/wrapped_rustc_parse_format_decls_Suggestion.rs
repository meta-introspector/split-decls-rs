use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub enum Suggestion {
    None,
    /// Replace inline argument with positional argument:
    /// `format!("{foo.bar}")` -> `format!("{}", foo.bar)`
    UsePositional,
    /// Remove `r#` from identifier:
    /// `format!("{r#foo}")` -> `format!("{foo}")`
    RemoveRawIdent(Range<usize>),
    /// Reorder format parameter:
    /// `format!("{foo:?#}")` -> `format!("{foo:#?}")`
    /// `format!("{foo:?x}")` -> `format!("{foo:x?}")`
    /// `format!("{foo:?X}")` -> `format!("{foo:X?}")`
    ReorderFormatParameter(Range<usize>, String),
}
