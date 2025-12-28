use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > Default for SmallVec < T , N > { # [inline] fn default () -> Self { Self :: new () } }
}