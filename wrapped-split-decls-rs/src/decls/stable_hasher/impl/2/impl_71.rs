use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , CTX > HashStable < CTX > for :: std :: ops :: RangeInclusive < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . start () . hash_stable (ctx , hasher) ; self . end () . hash_stable (ctx , hasher) ; } }
}