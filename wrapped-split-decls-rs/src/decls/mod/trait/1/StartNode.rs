use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait StartNode : DirectedGraph { fn start_node (& self) -> Self :: Node ; }
}