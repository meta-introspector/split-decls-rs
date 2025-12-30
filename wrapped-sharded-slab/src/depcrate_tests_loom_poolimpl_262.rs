// Generated macro for impl_262 (impl)
macro_rules! Depcrate_tests_loom_poolimpl_262 {
() => {
// Module: crate::tests::loom_pool
// Provides: {"impl_262"}
// Dependencies: {}
impl Clear for DontDropMe { fn clear (& mut self) { test_println ! ("-> DontDropMe clear: clearing data {:?}" , self . 0 . id) ; self . 0 . is_cleared . store (true , Ordering :: SeqCst) ; } }
};
}
