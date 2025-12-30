// Generated macro for impl_77 (impl)
macro_rules! Depcrate_internal_errorimpl_77 {
() => {
// Module: crate::internal::error
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "error")] impl < T : error :: Error + 'static > DowncastError for T { fn as_any (& self) -> & dyn Any { self } fn as_super (& self) -> & (dyn error :: Error + 'static) { self } }
};
}
