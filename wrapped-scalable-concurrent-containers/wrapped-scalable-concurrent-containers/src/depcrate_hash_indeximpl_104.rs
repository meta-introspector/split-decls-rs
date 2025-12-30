// Generated macro for impl_104 (impl)
macro_rules! Depcrate_hash_indeximpl_104 {
() => {
// Module: crate::hash_index
// Provides: {"impl_104"}
// Dependencies: {}
impl < K , V , H > Debug for Iter < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Iter") . field ("current_index" , & self . index) . field ("current_entry_ptr" , & self . entry_ptr) . finish () } }
};
}
