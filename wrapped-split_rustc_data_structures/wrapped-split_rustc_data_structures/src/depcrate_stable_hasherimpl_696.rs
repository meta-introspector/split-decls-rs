// Generated macro for impl_696 (impl)
macro_rules! Depcrate_stable_hasherimpl_696 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_696"}
// Dependencies: {}
impl < I : Idx , T , CTX > HashStable < CTX > for IndexVec < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }
};
}
