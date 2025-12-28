use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , K : Eq , V : Eq > Eq for SortedIndexMultiMap < I , K , V > { }
}