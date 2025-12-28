use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl MacroId { pub fn is_attribute (self , db : & dyn DefDatabase) -> bool { matches ! (self , MacroId :: ProcMacroId (it) if it . lookup (db) . kind == ProcMacroKind :: Attr) } }