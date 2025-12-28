use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "nightly")] impl < C > HashStable < C > for ExternAbi { # [inline] fn hash_stable (& self , _ : & mut C , hasher : & mut StableHasher) { Hash :: hash (self , hasher) ; } }