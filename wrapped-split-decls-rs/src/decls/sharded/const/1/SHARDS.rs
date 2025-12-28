use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const SHARDS : usize = 1 << SHARD_BITS ;
}