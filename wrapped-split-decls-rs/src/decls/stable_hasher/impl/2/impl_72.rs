use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T , CTX > HashStable < CTX > for IndexSlice < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }
}