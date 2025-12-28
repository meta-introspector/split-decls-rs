use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , R , CTX > HashStable < CTX > for indexmap :: IndexSet < K , R > where K : HashStable < CTX > + Eq + Hash , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for key in self { key . hash_stable (ctx , hasher) ; } } }
}