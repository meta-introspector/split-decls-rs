// Generated macro for impl_260 (impl)
macro_rules! Depcrate_tests_loom_poolimpl_260 {
() => {
// Module: crate::tests::loom_pool
// Provides: {"impl_260"}
// Dependencies: {}
impl DontDropMe { fn new (id : usize) -> (Arc < State > , Self) { let state = Arc :: new (State { is_dropped : AtomicBool :: new (false) , is_cleared : AtomicBool :: new (false) , id , }) ; (state . clone () , Self (state)) } }
};
}
