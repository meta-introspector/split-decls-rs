// Generated macro for impl_12 (impl)
macro_rules! Depcrate_aserrorimpl_12 {
() => {
// Module: crate::aserror
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > AsDynError < 'a > for dyn Error + Send + Sync + UnwindSafe + 'a { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
};
}
