// Generated macro for impl_81 (impl)
macro_rules! Depcrate_internal_errorimpl_81 {
() => {
// Module: crate::internal::error
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'v > From < & 'v (dyn error :: Error + 'static) > for ValueBag < 'v > { # [inline] fn from (v : & 'v (dyn error :: Error + 'static)) -> Self { ValueBag :: from_dyn_error (v) } }
};
}
