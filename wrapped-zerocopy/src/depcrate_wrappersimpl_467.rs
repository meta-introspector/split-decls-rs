// Generated macro for impl_467 (impl)
macro_rules! Depcrate_wrappersimpl_467 {
() => {
// Module: crate::wrappers
// Provides: {"impl_467"}
// Dependencies: {}
impl < T : ? Sized + KnownLayout > fmt :: Debug for MaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (core :: any :: type_name :: < Self > ()) } }
};
}
