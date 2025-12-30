// Generated macro for impl_47 (impl)
macro_rules! Depcrate_hash_cacheimpl_47 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_47"}
// Dependencies: {}
impl < K , V , H > Debug for OccupiedEntry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish_non_exhaustive () } }
};
}
