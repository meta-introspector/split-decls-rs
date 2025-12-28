use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > Default for LRUCache < T , N > { fn default () -> Self { Self :: new () } }
}