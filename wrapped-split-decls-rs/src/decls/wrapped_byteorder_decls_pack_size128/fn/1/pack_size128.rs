use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn pack_size128 (n : u128) -> usize { (16 - ((n | 1) . leading_zeros () >> 3)) as usize }