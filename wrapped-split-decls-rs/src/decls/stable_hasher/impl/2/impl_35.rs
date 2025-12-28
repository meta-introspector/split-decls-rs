use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for ! { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { unreachable ! () } }
}