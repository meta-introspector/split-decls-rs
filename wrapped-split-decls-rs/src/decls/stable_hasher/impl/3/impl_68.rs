use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T1 , T2 , CTX > HashStable < CTX > for Result < T1 , T2 > where T1 : HashStable < CTX > , T2 : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { mem :: discriminant (self) . hash_stable (ctx , hasher) ; match * self { Ok (ref x) => x . hash_stable (ctx , hasher) , Err (ref x) => x . hash_stable (ctx , hasher) , } } }
}