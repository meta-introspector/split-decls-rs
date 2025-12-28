use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Context { # [inline] fn default () -> Self { Self :: new () } }
}