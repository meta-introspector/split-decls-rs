// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { # [cfg (not (feature = "alloc"))] { None } # [cfg (feature = "alloc")] { self . source . as_ref () . map (| source | source . as_ref () as & (dyn core :: error :: Error + 'static)) } } }
};
}
