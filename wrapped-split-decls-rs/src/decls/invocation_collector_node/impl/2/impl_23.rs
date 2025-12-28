use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasNodeId for ExpandedCrate { fn node_id (& self) -> NodeId { self . 0 . id } fn node_id_mut (& mut self) -> & mut NodeId { & mut self . 0 . id } }