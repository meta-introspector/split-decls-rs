use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Subset of `ide_db::Definition` that doc links can resolve to."] pub enum DocLinkDef { ModuleDef (ModuleDef) , Field (Field) , SelfType (Trait) , }
}