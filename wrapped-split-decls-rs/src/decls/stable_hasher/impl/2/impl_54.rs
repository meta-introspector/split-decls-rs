use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < A , const N : usize , CTX > HashStable < CTX > for SmallVec < [A ; N] > where A : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }