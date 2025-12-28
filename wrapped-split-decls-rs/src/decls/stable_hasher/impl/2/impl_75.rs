use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < R : Idx , C : Idx , CTX > HashStable < CTX > for bit_set :: BitMatrix < R , C > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
}