use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns `true` if the graph has a cycle that is reachable from the start node."] pub fn is_cyclic < G > (graph : & G) -> bool where G : ? Sized + DirectedGraph + StartNode + Successors , { iterate :: TriColorDepthFirstSearch :: new (graph) . run_from_start (& mut iterate :: CycleDetector) . is_some () }