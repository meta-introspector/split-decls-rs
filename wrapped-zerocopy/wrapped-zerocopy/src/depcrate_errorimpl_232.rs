// Generated macro for impl_232 (impl)
macro_rules! Depcrate_errorimpl_232 {
() => {
// Module: crate::error
// Provides: {"impl_232"}
// Dependencies: {}
impl < Src , Dst : ? Sized + TryFromBytes > fmt :: Debug for ValidityError < Src , Dst > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ValidityError") . finish () } }
};
}
