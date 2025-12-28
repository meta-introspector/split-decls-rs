use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Buffer { # [inline] fn default () -> Buffer { Buffer :: new () } }
}