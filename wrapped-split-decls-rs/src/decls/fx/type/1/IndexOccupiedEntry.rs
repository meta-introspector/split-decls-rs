use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type IndexOccupiedEntry < 'a , K , V > = indexmap :: map :: OccupiedEntry < 'a , K , V > ;