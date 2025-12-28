use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type FxIndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;