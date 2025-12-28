use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : Eq + Hash > UnordCollection for UnordSet < V > { }
}