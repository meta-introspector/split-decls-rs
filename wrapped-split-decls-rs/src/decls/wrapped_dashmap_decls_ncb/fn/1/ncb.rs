use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn ncb (shard_amount : usize) -> usize { shard_amount . trailing_zeros () as usize }