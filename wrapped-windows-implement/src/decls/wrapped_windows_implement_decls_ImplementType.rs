use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,
    /// The best span for diagnostics.
    span: proc_macro2::Span,
}
