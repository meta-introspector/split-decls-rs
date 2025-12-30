// Generated macro for impl_917 (impl)
macro_rules! Depcrate_combinator_debug_internalsimpl_917 {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"impl_917"}
// Dependencies: {}
impl Drop for Depth { fn drop (& mut self) { if self . inc { let _ = DEPTH . fetch_sub (1 , std :: sync :: atomic :: Ordering :: SeqCst) ; } } }
};
}
