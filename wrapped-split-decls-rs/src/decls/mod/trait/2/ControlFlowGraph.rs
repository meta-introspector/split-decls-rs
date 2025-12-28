use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Alias for [`DirectedGraph`] + [`StartNode`] + [`Predecessors`] + [`Successors`]."] pub trait ControlFlowGraph : DirectedGraph + StartNode + Predecessors + Successors { }