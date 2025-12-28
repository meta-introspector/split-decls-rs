use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for String { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (hcx , hasher) ; } }
}