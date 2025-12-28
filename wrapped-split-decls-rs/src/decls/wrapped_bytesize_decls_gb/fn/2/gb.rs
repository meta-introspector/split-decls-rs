use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: gb");
# [doc = " Converts a quantity of gigabytes to bytes."] pub fn gb < V : Into < u64 > > (size : V) -> u64 { size . into () * GB }
}