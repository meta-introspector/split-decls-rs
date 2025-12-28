use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl OwnerId { # [inline] pub fn to_def_id (self) -> DefId { self . def_id . to_def_id () } }
}