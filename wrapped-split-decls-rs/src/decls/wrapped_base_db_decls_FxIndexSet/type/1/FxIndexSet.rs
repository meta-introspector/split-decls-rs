use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type FxIndexSet < T > = indexmap :: IndexSet < T , rustc_hash :: FxBuildHasher > ;