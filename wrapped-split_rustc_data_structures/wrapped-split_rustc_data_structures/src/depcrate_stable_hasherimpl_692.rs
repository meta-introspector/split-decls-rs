// Generated macro for impl_692 (impl)
macro_rules! Depcrate_stable_hasherimpl_692 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_692"}
// Dependencies: {}
impl < 'a , T , CTX > HashStable < CTX > for & 'a T where T : HashStable < CTX > + ? Sized , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
};
}
