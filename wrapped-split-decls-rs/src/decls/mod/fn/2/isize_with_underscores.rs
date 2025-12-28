use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Print an `isize` with underscore separators."] pub fn isize_with_underscores (n : isize) -> String { format_with_underscores (format ! ("{n}")) }