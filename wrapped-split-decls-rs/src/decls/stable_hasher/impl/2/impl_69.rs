use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T , CTX > HashStable < CTX > for & 'a T where T : HashStable < CTX > + ? Sized , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
}