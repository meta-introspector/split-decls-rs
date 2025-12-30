// Generated macro for impl_680 (impl)
macro_rules! Depcrate_stable_hasherimpl_680 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_680"}
// Dependencies: {}
impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: sync :: Arc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
};
}
