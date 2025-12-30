// Generated macro for impl_45 (impl)
macro_rules! Depcrate_hash_cacheimpl_45 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_45"}
// Dependencies: {}
impl < K , V , H > Debug for Entry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Vacant (v) => f . debug_tuple ("Entry") . field (v) . finish () , Self :: Occupied (o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
