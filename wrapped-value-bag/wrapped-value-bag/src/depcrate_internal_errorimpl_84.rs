// Generated macro for impl_84 (impl)
macro_rules! Depcrate_internal_errorimpl_84 {
() => {
// Module: crate::internal::error
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'v , 'u > From < & 'v & 'u (dyn error :: Error + 'static) > for ValueBag < 'v > where 'u : 'v , { # [inline] fn from (v : & 'v & 'u (dyn error :: Error + 'static)) -> Self { ValueBag :: from_dyn_error (* v) } }
};
}
