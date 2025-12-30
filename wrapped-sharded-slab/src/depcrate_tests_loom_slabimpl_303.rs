// Generated macro for impl_303 (impl)
macro_rules! Depcrate_tests_loom_slabimpl_303 {
() => {
// Module: crate::tests::loom_slab
// Provides: {"impl_303"}
// Dependencies: {}
impl AssertDropped { fn new (val : usize) -> (Self , SetDropped) { let dropped = std :: sync :: Arc :: new (AtomicBool :: new (false)) ; let val = SetDropped { val , dropped : dropped . clone () , } ; (Self { dropped } , val) } fn assert_dropped (& self) { assert ! (self . dropped . load (Ordering :: SeqCst) , "value should have been dropped!") ; } }
};
}
