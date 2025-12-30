// Generated macro for impl_697 (impl)
macro_rules! Depcrate_stable_hasherimpl_697 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_697"}
// Dependencies: {}
impl < I : Idx , CTX > HashStable < CTX > for DenseBitSet < I > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
};
}
