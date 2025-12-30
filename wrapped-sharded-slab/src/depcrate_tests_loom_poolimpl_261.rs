// Generated macro for impl_261 (impl)
macro_rules! Depcrate_tests_loom_poolimpl_261 {
() => {
// Module: crate::tests::loom_pool
// Provides: {"impl_261"}
// Dependencies: {}
impl Drop for DontDropMe { fn drop (& mut self) { test_println ! ("-> DontDropMe drop: dropping data {:?}" , self . 0 . id) ; self . 0 . is_dropped . store (true , Ordering :: SeqCst) } }
};
}
