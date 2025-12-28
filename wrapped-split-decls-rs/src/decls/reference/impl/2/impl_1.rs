use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'graph , G : DirectedGraph > DirectedGraph for & 'graph G { type Node = G :: Node ; fn num_nodes (& self) -> usize { (* * self) . num_nodes () } }
}