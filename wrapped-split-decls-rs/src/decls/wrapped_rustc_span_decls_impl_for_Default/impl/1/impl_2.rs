use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Span { fn default () -> Self { DUMMY_SP } }
}