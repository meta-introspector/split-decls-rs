use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type StdEntry < 'a , K , V > = std :: collections :: hash_map :: Entry < 'a , K , V > ;