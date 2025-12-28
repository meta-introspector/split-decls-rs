use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for FixedBitSet { fn default () -> Self { Self :: new () } }
}