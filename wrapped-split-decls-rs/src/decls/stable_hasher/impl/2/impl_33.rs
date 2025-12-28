use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for Hash128 { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { self . as_u128 () . hash (hasher) ; } }