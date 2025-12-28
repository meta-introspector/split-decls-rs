use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn should_skip_node (node : & tree_sitter :: Node , source : & str) -> bool { is_large_array (node , source , LARGE_ARRAY_THRESHOLD , LARGE_CONTENT_LENGTH_THRESHOLD) }