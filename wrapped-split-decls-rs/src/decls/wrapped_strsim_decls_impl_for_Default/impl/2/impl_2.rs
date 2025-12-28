use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < ValueType > Default for HybridGrowingHashmapChar < ValueType > where ValueType : Default + Clone + Copy + Eq , { fn default () -> Self { HybridGrowingHashmapChar { map : GrowingHashmapChar :: default () , extended_ascii : [Default :: default () ; 256] , } } }
}