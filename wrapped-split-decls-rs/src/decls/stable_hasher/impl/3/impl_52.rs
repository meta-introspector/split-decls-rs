use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V , R , CTX > HashStable < CTX > for indexmap :: IndexMap < K , V , R > where K : HashStable < CTX > + Eq + Hash , V : HashStable < CTX > , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for kv in self { kv . hash_stable (ctx , hasher) ; } } }
}