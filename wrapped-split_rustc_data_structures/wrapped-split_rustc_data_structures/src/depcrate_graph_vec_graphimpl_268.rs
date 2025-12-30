// Generated macro for impl_268 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_268 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_268"}
// Dependencies: {}
impl < N : Idx + Ord , const BR : bool > Successors for VecGraph < N , BR > { fn successors (& self , node : N) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }
};
}
