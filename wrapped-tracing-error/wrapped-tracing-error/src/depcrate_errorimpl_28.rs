// Generated macro for impl_28 (impl)
macro_rules! Depcrate_errorimpl_28 {
() => {
// Module: crate::error
// Provides: {"impl_28"}
// Dependencies: {}
impl Error for ErrorImpl < Erased > { fn source (& self) -> Option < & (dyn Error + 'static) > { self . error () . source () } }
};
}
