use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX : HashStableContext > ToStableHashKey < CTX > for ItemLocalId { type KeyType = ItemLocalId ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> ItemLocalId { * self } }
}