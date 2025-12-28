use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > stable_hasher :: HashStable < T > for Svh { # [inline] fn hash_stable (& self , ctx : & mut T , hasher : & mut stable_hasher :: StableHasher) { let Svh { hash } = * self ; hash . hash_stable (ctx , hasher) ; } }
}