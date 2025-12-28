use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl BitXorAssign < u64 > for Hash64 { # [inline] fn bitxor_assign (& mut self , rhs : u64) { self . inner ^= rhs ; } }
}