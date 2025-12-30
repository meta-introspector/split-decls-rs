// Generated macro for impl_136 (impl)
macro_rules! Depcrate_tracked_fnimpl_136 {
() => {
// Module: crate::tracked_fn
// Provides: {"impl_136"}
// Dependencies: {}
impl syn :: visit_mut :: VisitMut for ToDbLifetimeVisitor { fn visit_lifetime_mut (& mut self , i : & mut syn :: Lifetime) { i . clone_from (& self . db_lifetime) ; } }
};
}
