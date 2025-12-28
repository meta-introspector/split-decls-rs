use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Protocol > for c_int { fn from (p : Protocol) -> c_int { p . 0 } }
}