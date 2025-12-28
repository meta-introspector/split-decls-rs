use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ControlFlowGraph for T where T : DirectedGraph + StartNode + Predecessors + Successors { }