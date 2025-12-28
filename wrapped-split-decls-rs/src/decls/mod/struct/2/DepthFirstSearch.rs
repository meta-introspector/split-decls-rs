use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A \"depth-first search\" iterator for a directed graph."] pub struct DepthFirstSearch < G > where G : DirectedGraph + Successors , { graph : G , stack : Vec < G :: Node > , visited : DenseBitSet < G :: Node > , }