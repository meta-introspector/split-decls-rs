use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: counter_high");
# [inline] fn counter_high (counter : u64) -> u32 { (counter >> 32) as u32 }
}