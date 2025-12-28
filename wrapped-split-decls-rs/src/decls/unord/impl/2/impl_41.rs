use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < HCX , V : Hash + Eq + HashStable < HCX > > HashStable < HCX > for UnordBag < V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }