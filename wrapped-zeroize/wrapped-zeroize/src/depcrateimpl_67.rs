// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl < T , Z > AsMut < T > for Zeroizing < Z > where T : ? Sized , Z : AsMut < T > + Zeroize , { # [inline (always)] fn as_mut (& mut self) -> & mut T { self . 0 . as_mut () } }
};
}
