use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , rustc_hash :: FxBuildHasher > ;
}