use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for ByteSymbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_byte_str () . hash_stable (hcx , hasher) ; } }