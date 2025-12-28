use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A list of impl trait decls."] pub type ImplTraitDecls = Vec < ImplDef > ;
}