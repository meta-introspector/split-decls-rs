use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn counter_high (counter : u64) -> u32 { (counter >> 32) as u32 }