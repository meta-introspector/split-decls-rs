// Generated macro for impl_689 (impl)
macro_rules! Depcrate_stable_hasherimpl_689 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_689"}
// Dependencies: {}
impl < T , CTX > HashStable < CTX > for Option < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { if let Some (ref value) = * self { 1u8 . hash_stable (ctx , hasher) ; value . hash_stable (ctx , hasher) ; } else { 0u8 . hash_stable (ctx , hasher) ; } } }
};
}
