use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Type alias for a hash map that uses the Fx hashing algorithm."] # [cfg (feature = "std")] pub type FxHashMap < K , V > = HashMap < K , V , FxBuildHasher > ;
}