use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Print a `usize` with underscore separators."] pub fn usize_with_underscores (n : usize) -> String { format_with_underscores (format ! ("{n}")) }
}