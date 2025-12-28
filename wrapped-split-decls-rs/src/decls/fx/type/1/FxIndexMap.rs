use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;
}