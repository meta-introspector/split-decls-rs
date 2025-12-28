use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : AstIdNode > Eq for AssocItemLoc < N > { }
}