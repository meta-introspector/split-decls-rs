use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for :: std :: cmp :: Ordering { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* self as i8) . hash_stable (ctx , hasher) ; } }
}