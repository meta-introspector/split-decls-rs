use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : AstIdNode > Copy for AssocItemLoc < N > { }
}