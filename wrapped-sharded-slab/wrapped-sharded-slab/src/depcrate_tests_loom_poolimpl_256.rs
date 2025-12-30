// Generated macro for impl_256 (impl)
macro_rules! Depcrate_tests_loom_poolimpl_256 {
() => {
// Module: crate::tests::loom_pool
// Provides: {"impl_256"}
// Dependencies: {}
impl State { fn assert_clear (& self) { assert ! (! self . is_dropped . load (Ordering :: SeqCst)) ; assert ! (self . is_cleared . load (Ordering :: SeqCst)) ; } fn assert_not_clear (& self) { assert ! (! self . is_dropped . load (Ordering :: SeqCst)) ; assert ! (! self . is_cleared . load (Ordering :: SeqCst)) ; } }
};
}
