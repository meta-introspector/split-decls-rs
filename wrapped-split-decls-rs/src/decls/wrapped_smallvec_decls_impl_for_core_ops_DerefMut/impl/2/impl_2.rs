use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > core :: ops :: DerefMut for SmallVec < T , N > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
}