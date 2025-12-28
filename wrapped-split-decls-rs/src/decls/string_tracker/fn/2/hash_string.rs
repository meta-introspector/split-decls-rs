use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Hash a string for tracking"] fn hash_string (s : & str) -> u64 { let mut hasher = DefaultHasher :: new () ; s . hash (& mut hasher) ; hasher . finish () }
}