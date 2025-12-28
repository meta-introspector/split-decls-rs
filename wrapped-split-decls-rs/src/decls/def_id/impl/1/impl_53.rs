use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LocalModDefId { pub const CRATE_DEF_ID : Self = Self :: new_unchecked (CRATE_DEF_ID) ; }
}