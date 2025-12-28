use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ToBaseN for u128 { fn encoded_len (base : usize) -> usize { let mut max = u128 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u128 ; } len } }
}