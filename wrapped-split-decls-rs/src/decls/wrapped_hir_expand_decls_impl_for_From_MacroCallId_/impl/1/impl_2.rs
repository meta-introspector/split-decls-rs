use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < MacroCallId > for HirFileId { # [inline] fn from (file_id : MacroCallId) -> Self { HirFileId :: MacroFile (file_id) } }