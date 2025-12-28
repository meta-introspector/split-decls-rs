use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > AsRef < [T] > for SmallVec < T , N > { # [inline] fn as_ref (& self) -> & [T] { self . as_slice () } }
}