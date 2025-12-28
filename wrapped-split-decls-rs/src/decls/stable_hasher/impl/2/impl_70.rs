use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , CTX > HashStable < CTX > for :: std :: mem :: Discriminant < T > { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }