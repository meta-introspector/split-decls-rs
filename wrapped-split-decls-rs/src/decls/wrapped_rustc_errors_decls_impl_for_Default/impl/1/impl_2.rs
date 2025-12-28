use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Suggestions { fn default () -> Self { Self :: Enabled (vec ! []) } }
}