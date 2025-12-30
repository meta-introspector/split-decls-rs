// Generated macro for impl_190 (impl)
macro_rules! Depcrate_dep_graphimpl_190 {
() => {
// Module: crate::dep_graph
// Provides: {"impl_190"}
// Dependencies: {}
impl < T : HasDepContext , Q : Copy > HasDepContext for (T , Q) { type Deps = T :: Deps ; type DepContext = T :: DepContext ; fn dep_context (& self) -> & Self :: DepContext { self . 0 . dep_context () } }
};
}
