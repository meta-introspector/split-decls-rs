use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A trait for AST nodes having an ID."] pub trait HasNodeId { fn node_id (& self) -> NodeId ; fn node_id_mut (& mut self) -> & mut NodeId ; }