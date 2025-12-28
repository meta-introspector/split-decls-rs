use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Print an `f64` with precision 1 (one decimal place) and underscore separators."] pub fn f64p1_with_underscores (n : f64) -> String { format_with_underscores (format ! ("{n:.1}")) }
}