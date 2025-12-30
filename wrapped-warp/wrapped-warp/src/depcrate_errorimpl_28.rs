// Generated macro for impl_28 (impl)
macro_rules! Depcrate_errorimpl_28 {
() => {
// Module: crate::error
// Provides: {"impl_28"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (self . inner . as_ref ()) } }
};
}
