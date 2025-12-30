// Generated macro for impl_217 (impl)
macro_rules! Depcrate_parser_debugimpl_217 {
() => {
// Module: crate::parser::debug
// Provides: {"impl_217"}
// Dependencies: {}
impl DebugDepthGuard { pub (crate) fn new () -> Self { let depth = DEBUG_DEPTH . enter_unchecked () ; Self { depth , inc : true } } fn take (& mut self) -> Self { let depth = self . depth ; let inc = self . inc ; self . inc = false ; Self { depth , inc } } }
};
}
