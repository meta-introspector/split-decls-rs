use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MetaItemLit { pub fn value_str (& self) -> Option < Symbol > { LitKind :: from_token_lit (self . as_token_lit ()) . ok () . and_then (| lit | lit . str ()) } }
}