use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for Pu128 { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { { self . 0 } . hash_stable (ctx , hasher) } }