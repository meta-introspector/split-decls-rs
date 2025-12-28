use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn depth_first_search < G > (graph : G , from : G :: Node) -> iterate :: DepthFirstSearch < G > where G : Successors , { iterate :: DepthFirstSearch :: new (graph) . with_start_node (from) }