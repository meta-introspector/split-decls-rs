use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Ord , V > IntoIterator for SortedMap < K , V > { type Item = (K , V) ; type IntoIter = std :: vec :: IntoIter < (K , V) > ; fn into_iter (self) -> Self :: IntoIter { self . data . into_iter () } }
}