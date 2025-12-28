use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Copy + PartialEq > AppendOnlyVec < T > { pub fn contains (& self , val : T) -> bool { self . iter_enumerated () . any (| (_ , v) | v == val) } }
}