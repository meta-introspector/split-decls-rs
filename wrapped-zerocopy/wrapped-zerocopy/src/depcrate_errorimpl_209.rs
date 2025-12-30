// Generated macro for impl_209 (impl)
macro_rules! Depcrate_errorimpl_209 {
() => {
// Module: crate::error
// Provides: {"impl_209"}
// Dependencies: {}
impl < Src : Clone , Dst : ? Sized > Clone for AlignmentError < Src , Dst > { # [inline] fn clone (& self) -> Self { Self { src : self . src . clone () , _dst : SendSyncPhantomData :: default () } } }
};
}
