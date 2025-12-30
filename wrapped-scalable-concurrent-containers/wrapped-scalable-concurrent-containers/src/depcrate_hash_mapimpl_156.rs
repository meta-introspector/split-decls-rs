// Generated macro for impl_156 (impl)
macro_rules! Depcrate_hash_mapimpl_156 {
() => {
// Module: crate::hash_map
// Provides: {"impl_156"}
// Dependencies: {}
impl < K , V , H > Debug for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Reserve") . field (& self . additional) . finish () } }
};
}
