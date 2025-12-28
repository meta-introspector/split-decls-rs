use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn load_cached_matrix (cache_file : & str) -> Result < CachedMatrix > { let content = fs :: read_to_string (cache_file) ? ; let cached : CachedMatrix = serde_json :: from_str (& content) ? ; Ok (cached) }
}