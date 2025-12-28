use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Default maximum size."] # [doc = ""] # [doc = " Makes `ObjectIdentifier` 40-bytes total w\\ 1-byte length."] const DEFAULT_MAX_SIZE : usize = 39 ;