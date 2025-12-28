use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX : HashStableContext > HashStable < CTX > for LocalDefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (self . to_def_id ()) . local_hash () . hash_stable (hcx , hasher) ; } }