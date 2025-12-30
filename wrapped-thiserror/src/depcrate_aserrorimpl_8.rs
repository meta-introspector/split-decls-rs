// Generated macro for impl_8 (impl)
macro_rules! Depcrate_aserrorimpl_8 {
() => {
// Module: crate::aserror
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a , T : Error + 'a > AsDynError < 'a > for T { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
};
}
