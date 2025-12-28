use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: str_width");
pub fn str_width (s : & str) -> usize { s . chars () . map (char_width) . sum () }
}