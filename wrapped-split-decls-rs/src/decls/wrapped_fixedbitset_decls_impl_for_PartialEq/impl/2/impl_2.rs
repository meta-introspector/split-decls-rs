use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for FixedBitSet { fn eq (& self , other : & Self) -> bool { self . length == other . length && self . as_simd_slice () . eq (other . as_simd_slice ()) } }