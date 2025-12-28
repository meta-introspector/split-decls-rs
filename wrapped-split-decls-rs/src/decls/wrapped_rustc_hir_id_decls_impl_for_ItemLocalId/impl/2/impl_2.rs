use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ItemLocalId { # [doc = " Signal local id which should never be used."] pub const INVALID : ItemLocalId = ItemLocalId :: MAX ; }
}