use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : AstIdNode > PartialEq for AssocItemLoc < N > { fn eq (& self , other : & Self) -> bool { self . container == other . container && self . id == other . id } }
}