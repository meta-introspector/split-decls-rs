use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , I : Idx , T > IntoIterator for & 'a IndexSlice < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . raw . iter () } }