// Generated macro for impl_146 (impl)
macro_rules! Depcrate_hash_mapimpl_146 {
() => {
// Module: crate::hash_map
// Provides: {"impl_146"}
// Dependencies: {}
impl < K , V , H > Debug for OccupiedEntry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish_non_exhaustive () } }
};
}
