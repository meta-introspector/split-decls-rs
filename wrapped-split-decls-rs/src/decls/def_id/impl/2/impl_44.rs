use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX : HashStableContext > HashStable < CTX > for DefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (* self) . hash_stable (hcx , hasher) ; } }