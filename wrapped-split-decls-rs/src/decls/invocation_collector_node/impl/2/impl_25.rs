use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasNodeId for ExpandedTy { fn node_id (& self) -> NodeId { self . 0 . id } fn node_id_mut (& mut self) -> & mut NodeId { & mut self . 0 . id } }
}