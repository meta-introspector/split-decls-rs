use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SHARDS : usize = 1 << SHARD_BITS ;