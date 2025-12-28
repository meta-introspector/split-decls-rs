use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AttrItem { pub fn is_valid_for_outer_style (& self) -> bool { self . path == sym :: cfg_attr || self . path == sym :: cfg || self . path == sym :: forbid || self . path == sym :: warn || self . path == sym :: allow || self . path == sym :: deny } }
}