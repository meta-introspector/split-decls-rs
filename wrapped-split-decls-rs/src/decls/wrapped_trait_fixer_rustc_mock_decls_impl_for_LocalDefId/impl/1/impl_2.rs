use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LocalDefId { pub fn to_def_id (self) -> DefId { DefId } }
}