// Generated macro for impl_499 (impl)
macro_rules! Depcrate_collections_hash_setimpl_499 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_499"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < T , S > fmt :: Debug for Difference < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
