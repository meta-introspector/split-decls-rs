use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX : HashStableContext > HashStable < CTX > for CrateNum { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_def_id () . to_stable_hash_key (hcx) . stable_crate_id () . hash_stable (hcx , hasher) ; } }