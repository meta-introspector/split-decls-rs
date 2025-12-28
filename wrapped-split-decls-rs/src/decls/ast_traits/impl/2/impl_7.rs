use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : HasNodeId > HasNodeId for Box < T > { fn node_id (& self) -> NodeId { (* * self) . node_id () } fn node_id_mut (& mut self) -> & mut NodeId { (* * self) . node_id_mut () } }
}