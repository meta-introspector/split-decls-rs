use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq + Hash , V > UnordCollection for UnordMap < K , V > { }
}