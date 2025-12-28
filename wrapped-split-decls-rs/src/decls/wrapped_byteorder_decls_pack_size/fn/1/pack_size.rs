use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn pack_size (n : u64) -> usize { (8 - ((n | 1) . leading_zeros () >> 3)) as usize }
}