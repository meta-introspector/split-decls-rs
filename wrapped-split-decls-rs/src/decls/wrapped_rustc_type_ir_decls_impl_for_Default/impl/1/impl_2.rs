use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for UniverseIndex { fn default () -> Self { Self :: ROOT } }
}