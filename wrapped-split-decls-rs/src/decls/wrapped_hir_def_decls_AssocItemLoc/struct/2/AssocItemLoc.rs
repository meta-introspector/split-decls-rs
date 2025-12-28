use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct AssocItemLoc < N : AstIdNode > { pub container : ItemContainerId , pub id : AstId < N > , }
}