// Generated macro for impl_134 (impl)
macro_rules! Depcrate_graph_iterateimpl_134 {
() => {
// Module: crate::graph::iterate
// Provides: {"impl_134"}
// Dependencies: {}
impl < G > Iterator for DepthFirstSearch < G > where G : DirectedGraph + Successors , { type Item = G :: Node ; fn next (& mut self) -> Option < G :: Node > { let DepthFirstSearch { stack , visited , graph } = self ; let n = stack . pop () ? ; stack . extend (graph . successors (n) . filter (| & m | visited . insert (m))) ; Some (n) } }
};
}
