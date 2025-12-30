// Generated macro for impl_673 (impl)
macro_rules! Depcrate_stable_hasherimpl_673 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_673"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for [u8] { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; hasher . write (self) ; } }
};
}
