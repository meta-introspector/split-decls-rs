use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Computes `(a * b) >> 64`."] # [inline] fn mul_high_u64 (a : u64 , b : u64) -> u64 { (((a as u128) * (b as u128)) >> 64) as u64 }
}