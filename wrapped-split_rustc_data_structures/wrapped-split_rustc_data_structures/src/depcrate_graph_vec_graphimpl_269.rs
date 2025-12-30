// Generated macro for impl_269 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_269 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_269"}
// Dependencies: {}
impl < N : Idx + Ord > Predecessors for VecGraph < N , true > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors (node) . iter () . cloned () } }
};
}
