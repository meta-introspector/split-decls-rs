use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An array of cache-line aligned inner locked structures with convenience methods."] # [doc = " A single field is used when the compiler uses only one thread."] pub enum Sharded < T > { Single (Lock < T >) , Shards (Box < [CacheAligned < Lock < T > > ; SHARDS] >) , }