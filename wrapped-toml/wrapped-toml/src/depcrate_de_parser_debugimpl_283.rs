// Generated macro for impl_283 (impl)
macro_rules! Depcrate_de_parser_debugimpl_283 {
() => {
// Module: crate::de::parser::debug
// Provides: {"impl_283"}
// Dependencies: {}
impl DebugDepth { pub (crate) fn scoped (& self) -> DebugDepthGuard { DebugDepthGuard :: new () } pub (crate) fn enter_unchecked (& self) -> usize { self . 0 . fetch_add (1 , core :: sync :: atomic :: Ordering :: SeqCst) } pub (crate) fn exit_unchecked (& self) { let _ = self . 0 . fetch_sub (1 , core :: sync :: atomic :: Ordering :: SeqCst) ; } pub (crate) fn depth (& self) -> usize { self . 0 . load (core :: sync :: atomic :: Ordering :: SeqCst) } }
};
}
