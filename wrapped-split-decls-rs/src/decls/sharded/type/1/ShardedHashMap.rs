use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ShardedHashMap < K , V > = Sharded < HashTable < (K , V) > > ;