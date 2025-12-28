use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TestGraph { pub (super) fn new (start_node : usize , edges : & [(usize , usize)]) -> Self { let mut graph = TestGraph { num_nodes : start_node + 1 , start_node , successors : FxHashMap :: default () , predecessors : FxHashMap :: default () , } ; for & (source , target) in edges { graph . num_nodes = max (graph . num_nodes , source + 1) ; graph . num_nodes = max (graph . num_nodes , target + 1) ; graph . successors . entry (source) . or_default () . push (target) ; graph . predecessors . entry (target) . or_default () . push (source) ; } for node in 0 .. graph . num_nodes { graph . successors . entry (node) . or_default () ; graph . predecessors . entry (node) . or_default () ; } graph } }
}