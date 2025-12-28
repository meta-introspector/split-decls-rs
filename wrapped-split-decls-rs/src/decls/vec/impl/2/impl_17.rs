use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > IntoIterator for IndexVec < I , T > { type Item = T ; type IntoIter = vec :: IntoIter < T > ; # [inline] fn into_iter (self) -> vec :: IntoIter < T > { self . raw . into_iter () } }