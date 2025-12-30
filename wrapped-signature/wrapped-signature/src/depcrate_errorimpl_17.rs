// Generated macro for impl_17 (impl)
macro_rules! Depcrate_errorimpl_17 {
() => {
// Module: crate::error
// Provides: {"impl_17"}
// Dependencies: {}
impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { # [cfg (feature = "alloc")] { self . source . as_ref () . map (| source | source . as_ref () as & (dyn core :: error :: Error + 'static)) } # [cfg (not (feature = "alloc"))] { None } } }
};
}
