use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < usize > for MaxReached { fn eq (& self , other : & usize) -> bool { & self . 0 == other } }
}