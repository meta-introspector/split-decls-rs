use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type IndexEntry < 'a , K , V > = indexmap :: map :: Entry < 'a , K , V > ;