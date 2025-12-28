use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX , T : HashStable < CTX > > HashStable < CTX > for Steal < T > { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . borrow () . hash_stable (hcx , hasher) ; } }
}