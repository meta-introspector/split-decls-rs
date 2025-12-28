use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: gib");
# [doc = " Converts a quantity of gibibytes to bytes."] pub fn gib < V : Into < u64 > > (size : V) -> u64 { size . into () * GIB }
}