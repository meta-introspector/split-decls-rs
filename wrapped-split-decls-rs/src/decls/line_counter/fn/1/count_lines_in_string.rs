use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn count_lines_in_string (content : & str) -> usize { content . lines () . count () }
}