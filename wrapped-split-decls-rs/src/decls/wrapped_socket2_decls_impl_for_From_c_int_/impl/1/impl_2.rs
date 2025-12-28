use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < c_int > for Protocol { fn from (p : c_int) -> Protocol { Protocol (p) } }
}