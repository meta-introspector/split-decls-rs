use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Hash , const N : usize > Hash for SmallVec < T , N > { fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
}