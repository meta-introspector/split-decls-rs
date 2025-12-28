use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < ItemInNs > for ScopeDef { fn from (item : ItemInNs) -> Self { match item { ItemInNs :: Types (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Values (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Macros (id) => ScopeDef :: ModuleDef (ModuleDef :: Macro (id)) , } } }
}