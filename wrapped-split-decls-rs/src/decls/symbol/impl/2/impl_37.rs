use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > ToStableHashKey < CTX > for Symbol { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> String { self . as_str () . to_string () } }
}