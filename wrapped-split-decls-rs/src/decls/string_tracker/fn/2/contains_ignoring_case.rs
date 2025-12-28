use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Case-insensitive substring search"] pub fn contains_ignoring_case (haystack : & str , needle : & str) -> bool { haystack . to_lowercase () . contains (& needle . to_lowercase ()) }