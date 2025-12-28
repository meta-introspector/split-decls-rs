use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < u128 > for Pu128 { # [inline] fn eq (& self , other : & u128) -> bool { ({ self . 0 }) == * other } }
}