// Generated macro for impl_190 (impl)
macro_rules! Depcrate_graph_reversedimpl_190 {
() => {
// Module: crate::graph::reversed
// Provides: {"impl_190"}
// Dependencies: {}
impl < G : Successors > Predecessors for ReversedGraph < G > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . successors (node) } }
};
}
