// Generated macro for impl_264 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_264 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_264"}
// Dependencies: {}
impl < N : Idx + Ord > VecGraph < N , true > { # [doc = " Gets the predecessors for `target` as a slice."] pub fn predecessors (& self , target : N) -> & [N] { assert ! (target . index () < self . num_nodes ()) ; let target = N :: new (target . index () + self . num_nodes ()) ; let start_index = self . node_starts [target] ; let end_index = self . node_starts [target . plus (1)] ; & self . edge_targets [start_index .. end_index] } }
};
}
