use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ControlFlowGraph for T where T : DirectedGraph + StartNode + Predecessors + Successors { }
}