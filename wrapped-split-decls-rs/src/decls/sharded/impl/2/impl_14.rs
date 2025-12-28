use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq , V > ShardedHashMap < K , V > { pub fn with_capacity (cap : usize) -> Self { Self :: new (| | HashTable :: with_capacity (cap)) } pub fn len (& self) -> usize { self . lock_shards () . map (| shard | shard . len ()) . sum () } }
}