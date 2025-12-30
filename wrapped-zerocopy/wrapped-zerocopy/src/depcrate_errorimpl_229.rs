// Generated macro for impl_229 (impl)
macro_rules! Depcrate_errorimpl_229 {
() => {
// Module: crate::error
// Provides: {"impl_229"}
// Dependencies: {}
impl < Src : Clone , Dst : ? Sized + TryFromBytes > Clone for ValidityError < Src , Dst > { # [inline] fn clone (& self) -> Self { Self { src : self . src . clone () , _dst : SendSyncPhantomData :: default () } } }
};
}
