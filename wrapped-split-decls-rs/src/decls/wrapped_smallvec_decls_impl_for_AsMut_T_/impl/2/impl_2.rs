use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > AsMut < [T] > for SmallVec < T , N > { # [inline] fn as_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
}