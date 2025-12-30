// Generated macro for impl_107 (impl)
macro_rules! Depcrate_internal_fmtimpl_107 {
() => {
// Module: crate::internal::fmt
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'v , 'u > From < & 'v & 'u dyn Display > for ValueBag < 'v > where 'u : 'v , { # [inline] fn from (v : & 'v & 'u dyn Display) -> Self { ValueBag :: from_dyn_display (* v) } }
};
}
