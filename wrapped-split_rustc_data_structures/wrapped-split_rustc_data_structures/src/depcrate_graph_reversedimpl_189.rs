// Generated macro for impl_189 (impl)
macro_rules! Depcrate_graph_reversedimpl_189 {
() => {
// Module: crate::graph::reversed
// Provides: {"impl_189"}
// Dependencies: {}
impl < G : Predecessors > Successors for ReversedGraph < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . predecessors (node) } }
};
}
