// Generated macro for impl_170 (impl)
macro_rules! Depcrate_errorsimpl_170 {
() => {
// Module: crate::errors
// Provides: {"impl_170"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . source . as_ref () . map (| c | & * * c as & (dyn StdError + 'static)) } }
};
}
