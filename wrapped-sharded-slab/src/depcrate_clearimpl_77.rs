// Generated macro for impl_77 (impl)
macro_rules! Depcrate_clearimpl_77 {
() => {
// Module: crate::clear
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (all (loom , test))] impl < T : Clear > Clear for crate :: sync :: alloc :: Track < T > { fn clear (& mut self) { self . get_mut () . clear () } }
};
}
