// Generated macro for impl_463 (impl)
macro_rules! Depcrate_wrappersimpl_463 {
() => {
// Module: crate::wrappers
// Provides: {"impl_463"}
// Dependencies: {}
impl < T : Unaligned + Debug > Debug for Unalign < T > { # [inline (always)] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Debug :: fmt (self . deref () , f) } }
};
}
