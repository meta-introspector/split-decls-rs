use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T1 : HashStable < CTX > , T2 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 , T2) { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; } }