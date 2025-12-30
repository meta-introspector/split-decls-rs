// Generated macro for impl_98 (impl)
macro_rules! Depcrate_hash_indeximpl_98 {
() => {
// Module: crate::hash_index
// Provides: {"impl_98"}
// Dependencies: {}
impl < K , V , H > Debug for VacantEntry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
