use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for HierarchicalLayer { fn default () -> Self { Self :: new (2) } }
}