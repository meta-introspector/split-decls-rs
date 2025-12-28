use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for str { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . as_bytes () . hash_stable (ctx , hasher) ; } }
}