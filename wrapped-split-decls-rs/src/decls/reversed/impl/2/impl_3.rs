use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < G : DirectedGraph > DirectedGraph for ReversedGraph < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . inner . num_nodes () } }
}