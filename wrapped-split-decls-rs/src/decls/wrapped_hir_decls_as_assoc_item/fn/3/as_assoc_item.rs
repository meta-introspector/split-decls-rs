use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn as_assoc_item < 'db , ID , DEF , LOC > (db : & (dyn HirDatabase + 'db) , ctor : impl FnOnce (DEF) -> AssocItem , id : ID ,) -> Option < AssocItem > where ID : Lookup < Database = dyn DefDatabase , Data = AssocItemLoc < LOC > > , DEF : From < ID > , LOC : AstIdNode , { match id . lookup (db) . container { ItemContainerId :: TraitId (_) | ItemContainerId :: ImplId (_) => Some (ctor (DEF :: from (id))) , ItemContainerId :: ModuleId (_) | ItemContainerId :: ExternBlockId (_) => None , } }
}