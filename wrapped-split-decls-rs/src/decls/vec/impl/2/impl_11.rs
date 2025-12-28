use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > Deref for IndexVec < I , T > { type Target = IndexSlice < I , T > ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
}