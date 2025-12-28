use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A directed graph, efficient for cases where node indices are pre-existing."] # [doc = ""] # [doc = " If `BR` is true, the graph will store back-references, allowing you to get predecessors."] pub struct VecGraph < N : Idx , const BR : bool = false > { # [doc = " Indices into `edge_targets` that signify a start of list of edges."] node_starts : IndexVec < N , usize > , # [doc = " Targets (or sources for back refs) of edges"] edge_targets : Vec < N > , }