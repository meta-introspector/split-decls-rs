use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Hasher { fn default () -> Self { Self :: new () } }
}