use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : HashStable < CTX > + StableOrd , V : HashStable < CTX > , CTX > HashStable < CTX > for SortedMap < K , V > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . data . hash_stable (ctx , hasher) ; } }
}