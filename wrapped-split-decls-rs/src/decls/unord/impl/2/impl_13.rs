use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T : Copy + 'a , I : Iterator < Item = & 'a T > > UnordItems < & 'a T , I > { # [inline] pub fn copied (self) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . copied ()) } }
}