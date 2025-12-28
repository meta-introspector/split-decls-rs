use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX > HashStable < CTX > for LazyAttrTokenStream { fn hash_stable (& self , _hcx : & mut CTX , _hasher : & mut StableHasher) { panic ! ("Attempted to compute stable hash for LazyAttrTokenStream") ; } }