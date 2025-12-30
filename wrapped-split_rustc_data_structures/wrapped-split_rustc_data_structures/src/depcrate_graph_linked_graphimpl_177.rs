// Generated macro for impl_177 (impl)
macro_rules! Depcrate_graph_linked_graphimpl_177 {
() => {
// Module: crate::graph::linked_graph
// Provides: {"impl_177"}
// Dependencies: {}
impl < E > Edge < E > { pub fn source (& self) -> NodeIndex { self . source } pub fn target (& self) -> NodeIndex { self . target } pub fn source_or_target (& self , direction : Direction) -> NodeIndex { if direction == OUTGOING { self . target } else { self . source } } }
};
}
