use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefPathHash { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> DefPathHash { * self } }