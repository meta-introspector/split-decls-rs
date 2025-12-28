use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'graph , G : StartNode > StartNode for & 'graph G { fn start_node (& self) -> Self :: Node { (* * self) . start_node () } }
}