// Generated macro for impl_26 (impl)
macro_rules! Depcrate_layout_dfaimpl_26 {
() => {
// Module: crate::layout::dfa
// Provides: {"impl_26"}
// Dependencies: {}
impl State { pub (crate) fn new () -> Self { static COUNTER : AtomicU32 = AtomicU32 :: new (0) ; Self (COUNTER . fetch_add (1 , Ordering :: SeqCst)) } }
};
}
