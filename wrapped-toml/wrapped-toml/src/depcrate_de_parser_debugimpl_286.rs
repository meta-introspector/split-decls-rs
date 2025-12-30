// Generated macro for impl_286 (impl)
macro_rules! Depcrate_de_parser_debugimpl_286 {
() => {
// Module: crate::de::parser::debug
// Provides: {"impl_286"}
// Dependencies: {}
impl DebugDepthGuard { pub (crate) fn new () -> Self { let depth = DEBUG_DEPTH . enter_unchecked () ; Self { depth , inc : true } } fn take (& mut self) -> Self { let depth = self . depth ; let inc = self . inc ; self . inc = false ; Self { depth , inc } } }
};
}
