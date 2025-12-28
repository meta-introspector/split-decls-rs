use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn has_ignore_attr (attrs : & [syn :: Attribute] , name : & 'static str , meta : & 'static str) -> bool { let mut ignored = false ; attrs . iter () . for_each (| attr | { if ! attr . path () . is_ident (name) { return ; } let _ = attr . parse_nested_meta (| nested | { if nested . path . is_ident (meta) { ignored = true ; } Ok (()) }) ; }) ; ignored }
}