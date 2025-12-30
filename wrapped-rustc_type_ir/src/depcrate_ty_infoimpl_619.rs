// Generated macro for impl_619 (impl)
macro_rules! Depcrate_ty_infoimpl_619 {
() => {
// Module: crate::ty_info
// Provides: {"impl_619"}
// Dependencies: {}
impl < T : Hash > Hash for WithCachedTypeInfo < T > { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { # [cfg (feature = "nightly")] if self . stable_hash != Fingerprint :: ZERO { return self . stable_hash . hash (s) ; } self . internee . hash (s) } }
};
}
