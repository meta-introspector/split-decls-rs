// Generated macro for impl_464 (impl)
macro_rules! Depcrate_wrappersimpl_464 {
() => {
// Module: crate::wrappers
// Provides: {"impl_464"}
// Dependencies: {}
impl < T : Unaligned + Display > Display for Unalign < T > { # [inline (always)] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self . deref () , f) } }
};
}
