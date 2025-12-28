use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for [u8] { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; hasher . write (self) ; } }