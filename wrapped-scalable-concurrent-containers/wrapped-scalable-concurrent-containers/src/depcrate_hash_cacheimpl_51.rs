// Generated macro for impl_51 (impl)
macro_rules! Depcrate_hash_cacheimpl_51 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_51"}
// Dependencies: {}
impl < K , V , H > Debug for VacantEntry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
