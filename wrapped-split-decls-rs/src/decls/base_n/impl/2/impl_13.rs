use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ToBaseN for u32 { fn encoded_len (base : usize) -> usize { let mut max = u32 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u32 ; } len } }