// Generated macro for impl_660 (impl)
macro_rules! Depcrate_stable_hasherimpl_660 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_660"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for NonZero < u32 > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }
};
}
