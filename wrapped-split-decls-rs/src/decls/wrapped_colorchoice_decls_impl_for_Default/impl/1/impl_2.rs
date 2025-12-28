use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for AtomicChoice { fn default () -> Self { Self :: new () } }
}