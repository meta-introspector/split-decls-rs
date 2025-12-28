use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialOrd < u128 > for Pu128 { # [inline] fn partial_cmp (& self , other : & u128) -> Option < Ordering > { { self . 0 } . partial_cmp (other) } }
}