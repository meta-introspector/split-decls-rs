use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Hash for ThinVec < T > where T : Hash , { fn hash < H > (& self , state : & mut H) where H : Hasher , { self [..] . hash (state) ; } }
}