use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > Eq for SmallVec < T , N > where T : Eq { }
}