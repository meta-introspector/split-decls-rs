use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: tib");
# [doc = " Converts a quantity of tebibytes to bytes."] pub fn tib < V : Into < u64 > > (size : V) -> u64 { size . into () * TIB }
}