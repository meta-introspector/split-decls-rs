use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash for FixedBitSet { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . length . hash (state) ; self . as_simd_slice () . hash (state) ; } }
}