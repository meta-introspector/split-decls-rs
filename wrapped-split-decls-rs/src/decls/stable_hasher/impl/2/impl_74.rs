use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , CTX > HashStable < CTX > for DenseBitSet < I > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
}