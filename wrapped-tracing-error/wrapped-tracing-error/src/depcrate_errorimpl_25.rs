// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl < E > Error for TracedError < E > where E : std :: error :: Error + 'static , { fn source < 'a > (& 'a self) -> Option < & 'a (dyn Error + 'static) > { let erased = unsafe { & * (& self . inner as * const ErrorImpl < E > as * const ErrorImpl < Erased >) } ; Some (erased) } }
};
}
