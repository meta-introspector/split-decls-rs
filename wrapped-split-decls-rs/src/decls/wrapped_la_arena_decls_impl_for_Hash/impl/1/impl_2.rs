use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Hash for Idx < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) ; } }
}