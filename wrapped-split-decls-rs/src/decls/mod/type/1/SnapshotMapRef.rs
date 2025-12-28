use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type SnapshotMapRef < 'a , K , V , L > = SnapshotMap < K , V , & 'a mut FxHashMap < K , V > , & 'a mut L > ;
}