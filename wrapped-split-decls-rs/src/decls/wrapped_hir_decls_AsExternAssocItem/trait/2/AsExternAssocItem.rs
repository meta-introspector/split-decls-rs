use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait AsExternAssocItem { fn as_extern_assoc_item (self , db : & dyn HirDatabase) -> Option < ExternAssocItem > ; }
}