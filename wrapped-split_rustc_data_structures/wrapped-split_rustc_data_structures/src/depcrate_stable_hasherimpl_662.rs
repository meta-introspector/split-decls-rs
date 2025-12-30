// Generated macro for impl_662 (impl)
macro_rules! Depcrate_stable_hasherimpl_662 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_662"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for f32 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u32 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }
};
}
