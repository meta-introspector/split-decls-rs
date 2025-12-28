use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , I : Idx , T > IntoIterator for & 'a IndexVec < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
}