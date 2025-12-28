use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SnapshotMapStorage < K , V > = SnapshotMap < K , V , FxHashMap < K , V > , () > ;