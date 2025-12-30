// Generated macro for impl_189 (impl)
macro_rules! Depcrate_dep_graphimpl_189 {
() => {
// Module: crate::dep_graph
// Provides: {"impl_189"}
// Dependencies: {}
impl < T : DepContext > HasDepContext for T { type Deps = T :: Deps ; type DepContext = Self ; fn dep_context (& self) -> & Self :: DepContext { self } }
};
}
