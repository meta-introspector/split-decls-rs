use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn count_lines_in_string (content : & str) -> usize { content . lines () . count () }