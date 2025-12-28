use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T1 , T2 , T3 , T4 , CTX > HashStable < CTX > for (T1 , T2 , T3 , T4) where T1 : HashStable < CTX > , T2 : HashStable < CTX > , T3 : HashStable < CTX > , T4 : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1 , ref _2 , ref _3) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; _2 . hash_stable (ctx , hasher) ; _3 . hash_stable (ctx , hasher) ; } }
}