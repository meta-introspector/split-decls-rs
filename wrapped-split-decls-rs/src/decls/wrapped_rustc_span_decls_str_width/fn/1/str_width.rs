use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn str_width (s : & str) -> usize { s . chars () . map (char_width) . sum () }
}