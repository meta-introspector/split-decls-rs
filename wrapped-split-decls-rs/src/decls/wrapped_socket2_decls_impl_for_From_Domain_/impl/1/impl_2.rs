use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Domain > for c_int { fn from (d : Domain) -> c_int { d . 0 } }
}