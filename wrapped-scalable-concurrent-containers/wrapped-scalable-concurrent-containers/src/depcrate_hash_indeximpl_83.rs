// Generated macro for impl_83 (impl)
macro_rules! Depcrate_hash_indeximpl_83 {
() => {
// Module: crate::hash_index
// Provides: {"impl_83"}
// Dependencies: {}
impl < K , V , H > Debug for HashIndex < K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let guard = Guard :: new () ; self . reclaim_memory (& guard) ; f . debug_map () . entries (self . iter (& guard)) . finish () } }
};
}
