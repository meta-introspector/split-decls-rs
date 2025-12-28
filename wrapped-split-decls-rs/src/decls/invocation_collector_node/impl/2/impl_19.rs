use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasNodeId for ExpandedItem { fn node_id (& self) -> NodeId { self . 0 . id } fn node_id_mut (& mut self) -> & mut NodeId { & mut self . 0 . id } }
}