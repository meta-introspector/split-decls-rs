use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < CTX , T > HashStable < CTX > for PhantomData < T > { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { } }