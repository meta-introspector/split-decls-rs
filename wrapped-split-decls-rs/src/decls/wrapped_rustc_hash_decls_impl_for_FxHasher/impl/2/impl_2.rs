use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FxHasher { # [inline] fn add_to_hash (& mut self , i : usize) { self . hash = self . hash . wrapping_add (i) . wrapping_mul (K) ; } }
}