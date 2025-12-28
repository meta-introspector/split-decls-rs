use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: sync :: Arc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }