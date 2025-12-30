// Generated macro for impl_672 (impl)
macro_rules! Depcrate_stable_hasherimpl_672 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_672"}
// Dependencies: {}
impl < T : HashStable < CTX > , CTX > HashStable < CTX > for [T] { default fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for item in self { item . hash_stable (ctx , hasher) ; } } }
};
}
