use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for NonZero < usize > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }