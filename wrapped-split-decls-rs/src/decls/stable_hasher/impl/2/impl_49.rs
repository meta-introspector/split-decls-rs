use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : HashStable < CTX > , CTX > HashStable < CTX > for [T] { default fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for item in self { item . hash_stable (ctx , hasher) ; } } }