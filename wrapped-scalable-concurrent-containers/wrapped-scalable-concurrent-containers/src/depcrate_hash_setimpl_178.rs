// Generated macro for impl_178 (impl)
macro_rules! Depcrate_hash_setimpl_178 {
() => {
// Module: crate::hash_set
// Provides: {"impl_178"}
// Dependencies: {}
impl < K , H > Debug for HashSet < K , H > where K : Debug + Eq + Hash , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_set () ; self . iter_sync (| k | { d . entry (k) ; true }) ; d . finish () } }
};
}
