use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash128 { # [inline] pub fn new (n : u128) -> Self { Self { inner : n } } # [inline] pub fn truncate (self) -> Hash64 { Hash64 { inner : self . inner as u64 } } # [inline] pub fn wrapping_add (self , other : Self) -> Self { Self { inner : self . inner . wrapping_add (other . inner) } } # [inline] pub fn as_u128 (self) -> u128 { self . inner } }
}