use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LocalModDefId { pub fn is_top_level_module (self) -> bool { self . 0 . is_top_level_module () } }
}