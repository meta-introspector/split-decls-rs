use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , CTX > HashStable < CTX > for Interned < '_ , T > where T : HashStable < CTX > , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
}