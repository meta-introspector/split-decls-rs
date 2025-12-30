// Generated macro for impl_183 (impl)
macro_rules! Depcrate_graph_referenceimpl_183 {
() => {
// Module: crate::graph::reference
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'graph , G : Predecessors > Predecessors for & 'graph G { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . predecessors (node) } }
};
}
