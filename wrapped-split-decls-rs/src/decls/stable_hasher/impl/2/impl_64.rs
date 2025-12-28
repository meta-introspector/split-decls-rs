use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for bool { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (if * self { 1u8 } else { 0u8 }) . hash_stable (ctx , hasher) ; } }
}