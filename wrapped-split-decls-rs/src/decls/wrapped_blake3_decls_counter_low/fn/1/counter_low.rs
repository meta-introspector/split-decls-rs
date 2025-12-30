use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: counter_low");
# [inline] fn counter_low (counter : u64) -> u32 { counter as u32 }
}