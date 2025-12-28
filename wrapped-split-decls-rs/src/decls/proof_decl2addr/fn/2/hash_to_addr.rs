use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Hash to address (deterministic fallback)"] fn hash_to_addr (name : & str) -> u64 { let mut hasher = std :: collections :: hash_map :: DefaultHasher :: new () ; std :: hash :: Hasher :: write (& mut hasher , name . as_bytes ()) ; std :: hash :: Hasher :: finish (& hasher) }