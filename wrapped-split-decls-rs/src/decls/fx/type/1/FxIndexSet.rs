use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FxIndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;
}