use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S > Subtree < S > { # [doc = " Count the number of tokens recursively"] pub fn count (& self) -> usize { self . usize_len () } }
}