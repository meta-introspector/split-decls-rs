use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Hasher { # [inline] fn default () -> Self { Self :: new () } }
}