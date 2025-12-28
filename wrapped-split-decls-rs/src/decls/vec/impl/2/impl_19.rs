use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , I : Idx , T > IntoIterator for & 'a mut IndexVec < I , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; # [inline] fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
}