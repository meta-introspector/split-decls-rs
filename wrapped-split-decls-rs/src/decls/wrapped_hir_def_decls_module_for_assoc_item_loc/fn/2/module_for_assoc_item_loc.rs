use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn module_for_assoc_item_loc < 'db > (db : & (dyn 'db + DefDatabase) , id : impl Lookup < Database = dyn DefDatabase , Data = AssocItemLoc < impl AstIdNode > > ,) -> ModuleId { id . lookup (db) . container . module (db) }
}