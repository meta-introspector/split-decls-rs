use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX : HashStableContext > ToStableHashKey < CTX > for LocalDefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }