use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn count_lines_in_file (path : & Path) -> Result < usize > { let content = std :: fs :: read_to_string (path) ? ; Ok (content . lines () . count ()) }
}