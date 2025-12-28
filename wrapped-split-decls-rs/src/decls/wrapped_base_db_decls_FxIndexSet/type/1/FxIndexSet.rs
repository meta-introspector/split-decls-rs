use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FxIndexSet < T > = indexmap :: IndexSet < T , rustc_hash :: FxBuildHasher > ;
}