use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for FxHasher { # [inline] fn default () -> FxHasher { Self :: default () } }
}