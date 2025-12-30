// Generated macro for impl_8 (impl)
macro_rules! Depcrate_debugimpl_8 {
() => {
// Module: crate::debug
// Provides: {"impl_8"}
// Dependencies: {}
impl DebugDepth { pub (crate) fn enter_unchecked (& self) -> usize { self . 0 . fetch_add (1 , core :: sync :: atomic :: Ordering :: SeqCst) } pub (crate) fn exit_unchecked (& self) { let _ = self . 0 . fetch_sub (1 , core :: sync :: atomic :: Ordering :: SeqCst) ; } pub (crate) fn depth (& self) -> usize { self . 0 . load (core :: sync :: atomic :: Ordering :: SeqCst) } }
};
}
