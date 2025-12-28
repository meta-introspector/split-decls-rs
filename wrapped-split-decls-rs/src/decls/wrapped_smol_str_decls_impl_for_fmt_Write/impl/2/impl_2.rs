use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Write for SmolStrBuilder { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } }
}