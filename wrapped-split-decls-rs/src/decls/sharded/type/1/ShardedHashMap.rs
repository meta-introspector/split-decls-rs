use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type ShardedHashMap < K , V > = Sharded < HashTable < (K , V) > > ;
}