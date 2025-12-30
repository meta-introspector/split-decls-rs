// Generated macro for impl_92 (impl)
macro_rules! Depcrate_hash_indeximpl_92 {
() => {
// Module: crate::hash_index
// Provides: {"impl_92"}
// Dependencies: {}
impl < K , V , H > Debug for Entry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Vacant (v) => f . debug_tuple ("Entry") . field (v) . finish () , Self :: Occupied (o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
