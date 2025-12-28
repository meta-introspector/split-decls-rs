use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MarkedAttrs { pub fn new () -> Self { MarkedAttrs (GrowableBitSet :: new_empty ()) } pub fn mark (& mut self , attr : & Attribute) { self . 0 . insert (attr . id) ; } pub fn is_marked (& self , attr : & Attribute) -> bool { self . 0 . contains (attr . id) } }
}