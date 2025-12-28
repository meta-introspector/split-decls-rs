use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct HybridGrowingHashmapChar < ValueType > { map : GrowingHashmapChar < ValueType > , extended_ascii : [ValueType ; 256] , }
}