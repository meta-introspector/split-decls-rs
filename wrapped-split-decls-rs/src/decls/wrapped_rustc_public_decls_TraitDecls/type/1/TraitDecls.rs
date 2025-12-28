use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A list of trait decls."] pub type TraitDecls = Vec < TraitDef > ;
}