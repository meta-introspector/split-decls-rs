// Generated macro for impl_82 (impl)
macro_rules! Depcrate_internal_errorimpl_82 {
() => {
// Module: crate::internal::error
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'v > From < Option < & 'v (dyn error :: Error + 'static) > > for ValueBag < 'v > { # [inline] fn from (v : Option < & 'v (dyn error :: Error + 'static) >) -> Self { ValueBag :: from_option (v) } }
};
}
