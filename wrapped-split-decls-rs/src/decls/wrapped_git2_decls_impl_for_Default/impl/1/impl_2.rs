use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for ReferenceFormat { fn default () -> Self { ReferenceFormat :: NORMAL } }
}