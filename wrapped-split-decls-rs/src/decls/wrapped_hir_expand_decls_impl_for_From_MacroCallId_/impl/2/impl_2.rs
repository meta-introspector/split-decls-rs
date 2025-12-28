use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < MacroCallId > for HirFileId { # [inline] fn from (file_id : MacroCallId) -> Self { HirFileId :: MacroFile (file_id) } }
}