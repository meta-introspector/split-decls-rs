use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for FixedBitSet { fn eq (& self , other : & Self) -> bool { self . length == other . length && self . as_simd_slice () . eq (other . as_simd_slice ()) } }
}