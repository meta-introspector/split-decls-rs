use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [doc (alias = "MacroFileId")] pub struct MacroCallId { pub loc : MacroCallLoc , }