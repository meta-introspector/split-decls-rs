use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : HashStable < CTX > , CTX > HashStable < CTX > for Vec < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }
}