use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: eb");
# [doc = " Converts a quantity of exabytes to bytes."] pub fn eb < V : Into < u64 > > (size : V) -> u64 { size . into () * EB }
}