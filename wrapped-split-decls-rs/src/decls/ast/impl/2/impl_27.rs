use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < CTX : rustc_span :: HashStableContext > HashStable < CTX > for Path { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . segments . len () . hash_stable (hcx , hasher) ; for segment in & self . segments { segment . ident . hash_stable (hcx , hasher) ; } } }
}