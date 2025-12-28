use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < span :: MacroCallId > for MacroCallId { # [inline] fn from (value : span :: MacroCallId) -> Self { MacroCallId :: from_id (value . 0) } }
}