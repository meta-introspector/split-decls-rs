use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: count_lines_in_string");
pub fn count_lines_in_string (content : & str) -> usize { content . lines () . count () }
}