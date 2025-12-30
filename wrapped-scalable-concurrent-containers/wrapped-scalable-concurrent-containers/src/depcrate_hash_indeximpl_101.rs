// Generated macro for impl_101 (impl)
macro_rules! Depcrate_hash_indeximpl_101 {
() => {
// Module: crate::hash_index
// Provides: {"impl_101"}
// Dependencies: {}
impl < K , V , H > Debug for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Reserve") . field (& self . additional) . finish () } }
};
}
