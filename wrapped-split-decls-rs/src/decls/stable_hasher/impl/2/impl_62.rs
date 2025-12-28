use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < HCX > ToStableHashKey < HCX > for String { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> Self :: KeyType { self . clone () } }
}