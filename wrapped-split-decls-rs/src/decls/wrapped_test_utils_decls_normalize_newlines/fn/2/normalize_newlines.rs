use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: normalize_newlines");
fn normalize_newlines (s : & str) -> String { s . replace ("\r\n" , "\n") }
}