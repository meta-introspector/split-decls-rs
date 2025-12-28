use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Type > for c_int { fn from (t : Type) -> c_int { t . 0 } }
}