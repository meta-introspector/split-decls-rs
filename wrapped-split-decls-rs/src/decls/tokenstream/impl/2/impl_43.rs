use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX > HashStable < CTX > for TokenStream where CTX : crate :: HashStableContext , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { for sub_tt in self . iter () { sub_tt . hash_stable (hcx , hasher) ; } } }
}