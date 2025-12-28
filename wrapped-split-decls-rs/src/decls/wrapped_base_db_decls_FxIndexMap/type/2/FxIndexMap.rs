use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , std :: hash :: BuildHasherDefault < rustc_hash :: FxHasher > > ;
}