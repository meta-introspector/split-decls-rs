use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DirectedGraph for TestGraph { type Node = usize ; fn num_nodes (& self) -> usize { self . num_nodes } }
}