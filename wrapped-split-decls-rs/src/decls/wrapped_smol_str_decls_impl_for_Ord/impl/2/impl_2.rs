use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Ord for SmolStr { fn cmp (& self , other : & SmolStr) -> Ordering { self . as_str () . cmp (other . as_str ()) } }
}